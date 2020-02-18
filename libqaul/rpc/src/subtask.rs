use {
    crossbeam_queue::SegQueue,
    failure::{Error, format_err},
    futures::{
        future::{Future, FutureExt},
        never::Never,
        task::{FutureObj, AtomicWaker, ArcWake, Spawn, SpawnError, Context, Poll, waker_ref},
    },
    std::{
        collections::BTreeMap,
        pin::Pin,
        sync::{Arc, Mutex, Weak, atomic::{AtomicUsize, Ordering::Relaxed}},
    },
};

/// A collection of futures that will be polled as if they were a single
/// future. 
///
/// This future will **never** return, even when all interior tasks
/// complete. However when all handles to the manager drop, so will all
/// internal tasks.
///
/// This exists to allow new tasks to be spawned in locations where we 
/// don't have access to a spawner for the executor. It is probably
/// better to spawn tasks on the executor if you can.
///
/// Notably, when this future is polled, **all** sub-tasks which have
/// been woken will be polled. This might take a while so this future
/// could present a bottleneck in some cases. As with futures in general
/// it is important that each task does not block for a long time as
/// this will block *every* task managed by the manager.
#[derive(Clone)]
pub struct SubTaskManager(Arc<SubTaskManagerInner>);

impl SubTaskManager {
    pub fn new() -> Self {
        Self(Arc::new(SubTaskManagerInner {
            tasks: Mutex::new(BTreeMap::new()),
            next_task: AtomicUsize::new(0),
            waker: AtomicWaker::new(),
            woken: SegQueue::new(),
        }))
    }

    /// Get a non-owning spawner for this manager. This spawner
    /// will not stop the manager from getting dropped. This is
    /// a good thing because it means you can pass the spawner into
    /// a task managed by this manager without running into reference
    /// loops but does mean you have to keep a copy of this manager around
    /// seperately from this spawner or else the manager will drop.
    pub fn spawner(&self) -> Spawner {
        Spawner(Arc::downgrade(&self.0))
    }
}

impl Future for SubTaskManager {
    type Output = Never;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // register the waker in case any subtasks wake before we're done
        self.0.waker.register(cx.waker());

        // poll all woken tasks
        let mut tasks = self.0.tasks.lock().unwrap();
        while let Ok(woken_task) = self.0.woken.pop() { 
            if let Some(mut task) = tasks.get_mut(&woken_task) {
                let waker = Arc::new(Waker {
                    task_id: woken_task,
                    manager: Arc::downgrade(&self.0),
                });
                let waker = waker_ref(&waker);
                if task.poll_unpin(&mut Context::from_waker(&waker)).is_ready() {
                    // if the task is done remove it from the map
                    tasks.remove(&woken_task);
                }
            }
        }

        Poll::Pending
    }
}

/// A spawner for spawning tasks on a manager
///
/// This does not keep the parent manager alive so please
/// make sure you keep a handle to that somewhere or else the manager
/// will shutdown and the spawner will error.
#[derive(Clone)]
pub struct Spawner(Weak<SubTaskManagerInner>);

impl Spawner {
    /// Get a handle to the manager this spawner spawns on
    pub fn manager(&self) -> Result<SubTaskManager, Error> {
        self.0.upgrade()
            .map(|inner| SubTaskManager(inner))
            .ok_or(format_err!(
                "Couldn't get a handle to the manager as it has been dropped"
            ))
    }
}

impl Spawn for Spawner {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        let manager = self.0.upgrade().ok_or(SpawnError::shutdown())?;

        // insert the task into the task map as the next non-used task id
        //
        // if the id's loop back around that's not a problem, it's likely
        // most of the early tasks have completed so we'll still find one 
        // pretty fast, but this is why checking there's not already a task
        // with that id is crucial
        let mut tasks = manager.tasks.lock().unwrap();
        let task_id = loop {
            let task_id = manager.next_task.fetch_add(1, Relaxed);
            if !tasks.contains_key(&task_id) { break task_id; } 
        };
        tasks.insert(task_id, future);

        // wake the task to drive it until it stalls
        manager.wake(task_id);

        Ok(())
    }

    fn status(&self) -> Result<(), SpawnError> {
        self.0.upgrade()
            .map(|_| ())
            .ok_or(SpawnError::shutdown())
    }
}

struct SubTaskManagerInner {
    /// An owned set of futures that have yet to complete
    tasks: Mutex<BTreeMap<usize, FutureObj<'static, ()>>>,
    /// The next availible task id
    next_task: AtomicUsize,
    /// The waker to notify when a sub-task wakes
    waker: AtomicWaker,
    /// The set of woken sub-tasks to poll when the manager
    /// is pulled
    woken: SegQueue<usize>,
}

impl SubTaskManagerInner {
    fn wake(&self, task_id: usize) {
        self.woken.push(task_id);
        self.waker.wake();
    }
}

/// A Waker for a sub-task. When woken this will add the sub-task to the 
/// woken queue and call the manager's waker
struct Waker {
    task_id: usize,
    manager: Weak<SubTaskManagerInner>,
}

impl ArcWake for Waker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // if the manager has been dropped we can't wake the task
        //
        // this is probably fine as the task being woken also no longer exists
        if let Some(manager) = arc_self.manager.upgrade() {
            manager.wake(arc_self.task_id);
        }
    }
}

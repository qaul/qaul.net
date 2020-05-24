use {
    conjoiner,
    crate::{
        ASC_NAME, Call, CallId, error, Result, tags,
    },
    libqaul::{ 
        helpers::Tag,
        services::MetadataMap,
        users::UserAuth,
        Qaul,
    },
    std::sync::Arc,
};

pub(crate) struct CallDirectory {
    qaul: Arc<Qaul>,
}

impl CallDirectory {
    pub(crate) fn new(qaul: Arc<Qaul>) -> Self {
        Self { qaul }
    }

    async fn get_inner(&self, user: UserAuth) -> Result<MetadataMap> {
        let mut map = self.qaul
            .services()
            .query(user, ASC_NAME, Tag::empty(tags::CALL_LIST))
            .await?;
        map.reverse();
        Ok(map.pop()
            .unwrap_or_else(|| MetadataMap::new(tags::CALL_LIST)))
    }

    pub(crate) async fn get_all(&self, user: UserAuth) -> Result<Vec<Call>> {
        Ok(self.get_inner(user)
            .await?
            .iter()
            .map(|(_, v)| conjoiner::deserialise(v).unwrap())
            .collect())
    }

    pub(crate) async fn get(&self, user: UserAuth, id: CallId) -> Result<Call> {
        self.get_inner(user)
            .await?
            .iter()
            .fold(Err(error::NoSuchCall(id).into()), |opt, (this_id, vec)| {
                opt.or_else(|prev| {
                    if this_id == &id.to_string() {
                        Ok(conjoiner::deserialise(vec).unwrap())
                    } else {
                        Err(prev)
                    }
                })
            })
    }

    pub(crate) async fn insert(&self, user: UserAuth, call: &Call) -> Result<()> {
        self.qaul
            .services()
            .save(
                user.clone(),
                ASC_NAME,
                self.get_inner(user)
                    .await?
                    .add(call.id.to_string(), conjoiner::serialise(call).unwrap()),
                Tag::empty(tags::CALL_LIST),
            )
            .await?;

        Ok(())
    }

    pub(crate) async fn update<F>(&self, user: UserAuth, id: CallId, f: F) -> Result<Call> 
    where
        F: FnOnce(Call) -> Call
    {
        let call = self.get(user.clone(), id).await?;
        let call = f(call);
        self.insert(user, &call).await?;
        Ok(call)
    }
}

use {
    async_std::{
        stream::interval, 
        sync::{Mutex, RwLock},
    },
    crate::{
        ASC_NAME, Call, CallId, CallMessage, Voice, CallUser, 
        CallEvent, StreamState, VoiceDataPacket, VoiceData, CallData,
    },
    conjoiner,
    futures::{
        sink::SinkExt,
        stream::StreamExt,
    },
    libqaul::{
        helpers::TagSet,
        messages::ID_LEN,
        users::UserAuth,
        Identity,
    },
    opus::{Decoder, Channels},
    rubato::Resampler,
    std::{
        collections::BTreeMap,
        sync::Arc,
        time::{Duration, Instant},
    },
};

/// The worker that handles incoming messages for a client
#[tracing::instrument(skip(voice), level = "info")]
pub(crate) async fn client_message_worker(user: Identity, voice: Arc<Voice>) {
    let user = { Arc::clone(voice.users.read().await.get(&user).unwrap()) };

    // subscribe to incoming messages
    let sub = voice.qaul
        .messages()
        .subscribe(user.auth.clone(), ASC_NAME, TagSet::empty())
        .await
        .unwrap();
    trace!("Creating message subscription!");

    loop {
        let msg = sub.next().await;

        // skip messages we've sent
        if msg.sender == user.auth.0 {
            continue;
        }

        // call id is carried in a tag so grab and deserialize that
        let id = msg.tags
            .iter()
            .filter(|tag| tag.key == "call-id")
            .filter(|tag| tag.val.len() != ID_LEN)
            .map(|tag| Identity::from_bytes(&tag.val))
            .next();
        let id: CallId = match id {
            Some(id) => id,
            None => {
                warn!("Call message recieved with no call id tag");
                continue;
            },
        };

        // next match on the message payload
        match conjoiner::deserialise(&msg.payload) {

            // if we have been invited to a call...
            Ok(CallMessage::Invitation(inv)) => {
                info!("Received invitation to {:?}", id);

                // add the call to our call database
                let mut call = Call {
                    id,
                    participants: inv.participants,
                    invitees: inv.invitees,
                };
                call.invitees.insert(user.auth.0);
                let res = voice.calls
                    .lock()
                    .await
                    .insert(user.auth.clone(), &call)
                    .await;
                if let Err(e) = res {
                    warn!("Failed to insert new call into directory (this might be due to the client exiting?): {}", e);
                }

                // then notify any invitation subscriptions that we've been invited
                let mut subs = user.invitation_subs.write().await;
                // oh how i long for `drain_filter`
                let mut i = 0;
                while i != subs.len() {
                    if let Err(_) = subs[i].send(call.clone()).await {
                        subs.remove(i);
                    } else {
                        i += 1;
                    }
                }
            },

            // if someone else has been invited to a call...
            Ok(CallMessage::InvitationSent(to)) => {
                info!("Recieved invitation notification for user {:?} on call {:?}", to, id);

                // update the call in our database
                let res = voice.calls
                    .lock()
                    .await
                    .update(user.auth.clone(), id, |mut call| {
                        call.invitees.insert(to);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }

                // then push the event to any subscribers
                let event = CallEvent::UserInvited(to);
                if let Some(mut subs) = user.call_event_subs.write().await.get_mut(&id) {
                    let mut i = 0;
                    while i != subs.len() {
                        if let Err(_) = subs[i].send(event.clone()).await {
                            subs.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }
            },

            // if someone has joined the call...
            Ok(CallMessage::Join) => {
                // note who it is
                let joined_user = msg.sender;

                info!("Recieved join message for user {:?} on call {:?}", joined_user, id);

                // update the call in our database
                let res = voice.calls
                    .lock()
                    .await
                    .update(user.auth.clone(), id, |mut call| {
                        call.participants.insert(joined_user);
                        call.invitees.insert(joined_user);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }

                // and notify any subscribers
                let event = CallEvent::UserJoined(joined_user);
                if let Some(mut subs) = user.call_event_subs.write().await.get_mut(&id) {
                    let mut i = 0;
                    while i != subs.len() {
                        if let Err(_) = subs[i].send(event.clone()).await {
                            subs.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }
            },

            // if a user has left a call
            Ok(CallMessage::Part) => {
                // note who they are
                let parting_user = msg.sender;

                info!("Receieved part message for user {:?} on call {:?}", parting_user, id);

                // update our call database
                let res = voice.calls
                    .lock()
                    .await
                    .update(user.auth.clone(), id, |mut call| {
                        call.participants.remove(&parting_user);
                        call.invitees.remove(&parting_user);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }

                // then notify any subscribers
                let event = CallEvent::UserParted(parting_user);
                if let Some(mut subs) = user.call_event_subs.write().await.get_mut(&id) {
                    let mut i = 0;
                    while i != subs.len() {
                        if let Err(_) = subs[i].send(event.clone()).await {
                            subs.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }
            },

            // if a use is sending us voice data
            Ok(CallMessage::Data(data)) => {
                // check if we already know about the stream it's a part of
                let mut streams = user.incoming_streams.write().await;
                if let Some(state) = streams.get_mut(&data.stream) {
                    // also check it matches the user or else someone is fucking with
                    // our audio (how rude)
                    if state.user != msg.sender {
                        error!(
                            "Discarding packet due to mismatched sender ({} vs {}), someone might be trying to do something malicious!",
                            msg.sender,
                            state.user,
                        );
                        continue;
                    }

                    // if the stream is still in the startup window
                    if state.startup_timeout.is_some() {
                        // if the stream is still in the startup window roll the initial
                        // sequence number back if this packet comes earlier
                        //
                        // this makes sure we start at the earliest packet we have availible when
                        // the stream goes live
                        state.next_sequence_number = state.next_sequence_number
                            .min(data.sequence_number);
                    } else if state.next_sequence_number > data.sequence_number {
                        // otherwise discard this packet if it comes before the current
                        // sequence number
                        trace!(
                            "Discarding packet in stream {} with sequence number {} (current is {})", 
                            data.stream,
                            data.sequence_number,
                            state.next_sequence_number,
                        );
                        continue;
                    }

                    // then queue the packet in the jitter buffer 
                    state.jitter_buffer.insert(data.sequence_number, data.data);
                } else {
                    // if we don't know about the stream try to make a new decoder for it
                    let decoder = match Decoder::new(48000, Channels::Mono) {
                        Ok(decoder) => decoder,
                        Err(e) => {
                            warn!(
                                "Incoming data packet for stream {} failed to create decoder: {}", 
                                data.stream,
                                e,
                            );
                            continue;
                        },
                    };

                    // then build a jitter buffer and insert this packet
                    let mut jitter_buffer = BTreeMap::new();
                    jitter_buffer.insert(data.sequence_number, data.data);

                    // then build the stream state and add it to the stream map
                    let stream = StreamState {
                        call: id,
                        user: msg.sender,
                        jitter_buffer,
                        next_sequence_number: data.sequence_number,
                        // we will wait 250 milliseconds to get new packets before we start
                        // pulling them off
                        startup_timeout: Some(Instant::now() + Duration::from_millis(250)),
                        shutdown_timeout: None,
                        decoder: Mutex::new(decoder),
                    };
                    streams.insert(data.stream, stream);
                }
            },

            Err(_) => {
                warn!("Failed to deserialize message");
            }
        }
    }
}

/// The regular heartbeat of audio data processing
///
/// This handles both the encoding of outgoing voice packets and the decoding of incoming voice
/// packets
#[tracing::instrument(skip(voice), level = "info")]
pub async fn voice_worker(user: Identity, voice: Arc<Voice>) {
    let user = { Arc::clone(voice.users.read().await.get(&user).unwrap()) };
    trace!("Started voice worker");

    // we deal with 20ms intervals
    let mut interval = interval(Duration::from_millis(20));
    while let Some(_) = interval.next().await {
        let mut voice_data: BTreeMap<CallId, VoiceData> = BTreeMap::new();
        let mut streams = user.incoming_streams.write().await;
        // streams that should be deleted
        // this is a hack and i hate it
        let mut expired_streams = Vec::new();
        // to save us from calling it potentially a bunch down the line
        let now = Instant::now();

        // now for each stream
        for (id, state) in streams.iter_mut() {
            // if we're in the startup window don't do anything
            if state.startup_timeout.map(|time| now >= time).unwrap_or(false) {
                continue;
            }
            state.startup_timeout = None;

            // if the shutdown window has passed
            if state.shutdown_timeout.map(|time| now < time).unwrap_or(false) {
                // check if we can save the stream by making a time jump
                //
                // TODO: there's a better way to handle this i'm sure
                let (to_remove, to_keep) = state.jitter_buffer
                    .keys()
                    .partition::<Vec<_>, _>(
                        |sequence_number| **sequence_number < state.next_sequence_number
                    );

                if let Some(sequence_number) = to_keep.into_iter().min() {
                    // jump to the nearest packet after the time split
                    state.next_sequence_number = sequence_number;
                } else {
                    // if there's no voice packets in the future shut down the stream
                    expired_streams.push(*id);
                    continue;
                }

                // and remove the packets before the time split 
                to_remove
                    .into_iter()
                    .for_each(|sequence_number| {
                        state.jitter_buffer.remove(&sequence_number);
                    });
            }

            // get the next packet if it has been recieved
            let data = if let Some(packet) = state.jitter_buffer.remove(&state.next_sequence_number) {
                state.shutdown_timeout = None;
                state.next_sequence_number += 1;
                packet
            } else {
                // otherwise return a packet of silence
                if state.shutdown_timeout.is_none() {
                    state.shutdown_timeout = Some(now + Duration::from_secs(15));
                }
                Vec::new()
            };

            // next we decode the data packet
            let mut samples = vec![0.0; 48000 / 50];
            match state.decoder.lock().await.decode_float(&data, &mut samples, false) {
                Ok(index) => { samples.truncate(index); },
                Err(e) => {
                    warn!("Could not decode packet for stream {}: {}", id, e);
                    expired_streams.push(*id);
                    continue;
                },
            }

            // then we add it to this frame's data for it's call
            let packet = VoiceDataPacket {
                user: state.user,
                samples,
            };
            if let Some(call) = voice_data.get_mut(&state.call) {
                call.insert(*id, packet);
            } else {
                let mut call = BTreeMap::new();
                call.insert(*id, packet);
                voice_data.insert(state.call, call);
            }
        }

        // next clean up any expired streams
        for id in expired_streams.iter() {
            streams.remove(id);
        }

        std::mem::drop(streams);

        // then take each stream sub and dispatch it's voice data
        let mut stream_subs = user.stream_subs.write().await;

        let mut empty_calls = Vec::new();
        for (call, subs) in stream_subs.iter_mut() {
            let _default = BTreeMap::new();
            let frame = voice_data
                .get(call)
                .unwrap_or(&_default);

            let mut i = 0;
            while i != subs.len() {
                if let Err(_) = subs[i].send(frame.clone()).await {
                    subs.remove(i);
                } else {
                    i += 1;
                }
            }

            // queue any call with no subscribers up for removal
            if subs.len() == 0 {
                empty_calls.push(*call);
            }
        }

        // and remove any empty subscription lists
        for id in empty_calls.iter() {
            stream_subs.remove(id);
        }

        std::mem::drop(stream_subs);

        // now we deal with encoding
        let mut streams = user.outgoing_streams.write().await;

        let mut errored_streams = Vec::new();
        for (id, state) in streams.iter_mut() {
            let frames_needed = state.resampler.nbr_frames_needed();
            if state.samples.len() < frames_needed {
                warn!("Stream {} doesn't have enough samples to encode", id);
                continue;
            }

            let mut samples = Vec::with_capacity(frames_needed);
            for _ in 0..frames_needed {
                samples.push(state.samples.pop_front().unwrap());
            }

            let samples = match state.resampler.process(&[samples]) {
                Ok(samples) => samples,
                Err(e) => {
                    warn!("Could not resample packet for stream {}: {}", id, e);
                    errored_streams.push(*id);
                    continue;
                },
            };

            // encode the samples into a byte buffer
            let mut data = vec![0; 256];
            match state.encoder.lock().await.encode_float(&samples[0][..], &mut data) {
                Ok(index) => { data.truncate(index); },
                Err(e) => {
                    warn!("Could not encode packet for stream {}: {}", id, e);
                    errored_streams.push(*id);
                    continue;
                },
            }

            // construct the message and try to send it
            let msg = CallMessage::Data(CallData {
                stream: *id,
                data,
                sequence_number: state.next_sequence_number,
            });
            let call = match voice.get_call(user.auth.clone(), state.call).await {
                Ok(call) =>  call,
                Err(_) => {
                    warn!("Could not get call {} for stream {}", state.call, id);
                    errored_streams.push(*id);
                    continue;
                },
            };
            let res = msg.send_to(
                user.auth.clone(),
                &call.participants,
                state.call,
                &voice.qaul,
            ).await;
            
            if let Err(e) = res {
                warn!("Failed to send audio for stream {}: {}", id, e);
                errored_streams.push(*id);
                continue;
            } else {
                // if all goes well increment the sent sequence number
                state.next_sequence_number += 1;
            }
        }

        // and remove any broken streams 
        for id in errored_streams.iter() {
            streams.remove(id);
        }
    }
}

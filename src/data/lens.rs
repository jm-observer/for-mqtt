use crate::data::common::Broker;
use crate::data::common::{
    Msg, PublicInput, QoS, SubscribeHis, SubscribeInput, SubscribeTopic, TabStatus,
};
use crate::data::hierarchy::AppData;
use crate::data::AString;
use druid::im::Vector;
use druid::{Data, Lens};
use log::debug;

pub struct BrokerIndex(pub usize);

impl druid::Lens<AppData, Broker> for BrokerIndex {
    fn with<V, F: FnOnce(&Broker) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.find_broker(self.0) {
            Some(broker) => broker,
            None => {
                debug!("{}", self.0);
                unreachable!("{}", self.0)
            }
        })
    }
    fn with_mut<V, F: FnOnce(&mut Broker) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.brokers.iter_mut().find(|x| x.id == self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}
pub struct BrokerIndexLensVecSubscribeHis(pub usize);

impl druid::Lens<AppData, Vector<SubscribeHis>> for BrokerIndexLensVecSubscribeHis {
    fn with<V, F: FnOnce(&Vector<SubscribeHis>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_hises.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<SubscribeHis>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        f(match data.subscribe_hises.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

pub struct LensSelectedSubscribeHis;

impl druid::Lens<AppData, Vector<SubscribeHis>> for LensSelectedSubscribeHis {
    fn with<V, F: FnOnce(&Vector<SubscribeHis>) -> V>(&self, data: &AppData, f: F) -> V {
        if let Some(broker) = data.get_selected_broker() {
            f(match data.subscribe_hises.get(&broker.id) {
                Some(broker) => broker,
                None => unreachable!(""),
            })
        } else {
            let datas = Vector::new();
            f(&datas)
        }
    }
    fn with_mut<V, F: FnOnce(&mut Vector<SubscribeHis>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        let id = match data.get_selected_broker() {
            Some(broker) => broker.id,
            None => {
                let mut datas = Vector::new();
                return f(&mut datas);
            }
        };
        f(match data.subscribe_hises.get_mut(&id) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}
pub struct BrokerIndexLensVecSubscribeTopic(pub usize);

impl druid::Lens<AppData, Vector<SubscribeTopic>> for BrokerIndexLensVecSubscribeTopic {
    fn with<V, F: FnOnce(&Vector<SubscribeTopic>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_topics.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<SubscribeTopic>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        f(match data.subscribe_topics.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}
pub struct BrokerIndexLensVecMsg(pub usize);

impl druid::Lens<AppData, Vector<Msg>> for BrokerIndexLensVecMsg {
    fn with<V, F: FnOnce(&Vector<Msg>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.msgs.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Msg>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.msgs.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}
pub struct BrokerIndexLensSubscribeInput(pub usize);

impl druid::Lens<AppData, SubscribeInput> for BrokerIndexLensSubscribeInput {
    fn with<V, F: FnOnce(&SubscribeInput) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_input.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut SubscribeInput) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.subscribe_input.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

pub struct BrokerIndexLensPublicInput(pub usize);

impl druid::Lens<AppData, PublicInput> for BrokerIndexLensPublicInput {
    fn with<V, F: FnOnce(&PublicInput) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.public_input.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut PublicInput) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.public_input.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

pub struct BrokerIndexLensTabStatus(pub usize);

impl druid::Lens<AppData, TabStatus> for BrokerIndexLensTabStatus {
    fn with<V, F: FnOnce(&TabStatus) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.tab_statuses.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut TabStatus) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.tab_statuses.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

#[derive(Clone)]
pub struct DbIndex {
    pub data: AppData,
    pub id: usize,
}
impl druid::Data for DbIndex {
    fn same(&self, _other: &Self) -> bool {
        let self_status = match self.data.tab_statuses.get(&self.id) {
            Some(status) => status,
            None => return false,
        };
        let other_status = match _other.data.tab_statuses.get(&self.id) {
            Some(status) => status,
            None => return false,
        };
        Data::same(self_status, other_status)
    }
}

pub struct Index(pub usize);

impl druid::Lens<AppData, DbIndex> for Index {
    fn with<V, F: FnOnce(&DbIndex) -> V>(&self, data: &AppData, f: F) -> V {
        let db_index = DbIndex {
            data: data.clone(),
            id: self.0,
        };
        f(&db_index)
    }
    fn with_mut<V, F: FnOnce(&mut DbIndex) -> V>(&self, data: &mut AppData, f: F) -> V {
        let mut db_index = DbIndex {
            data: data.clone(),
            id: self.0,
        };
        f(&mut db_index)
    }
}

pub struct BrokerStoredList;

impl druid::Lens<AppData, Vector<Broker>> for BrokerStoredList {
    fn with<V, F: FnOnce(&Vector<Broker>) -> V>(&self, data: &AppData, f: F) -> V {
        let broker_list: Vector<Broker> = data
            .brokers
            .iter()
            .filter(|x| x.stored)
            .map(|x| x.clone())
            .collect();
        f(&broker_list)
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Broker>) -> V>(&self, data: &mut AppData, f: F) -> V {
        let mut broker_list: Vector<Broker> = data
            .brokers
            .iter()
            .filter(|x| x.stored)
            .map(|x| x.clone())
            .collect();
        f(&mut broker_list)
    }
}

pub struct MsgMsgLens;

impl Lens<Msg, AString> for MsgMsgLens {
    fn with<V, F: FnOnce(&AString) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.msg,
            Msg::Subscribe(msg) => &msg.msg,
        })
    }

    fn with_mut<V, F: FnOnce(&mut AString) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.msg,
            Msg::Subscribe(msg) => &mut msg.msg,
        })
    }
}

pub struct MsgTopicLens;

impl Lens<Msg, AString> for MsgTopicLens {
    fn with<V, F: FnOnce(&AString) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.topic,
            Msg::Subscribe(msg) => &msg.topic,
        })
    }

    fn with_mut<V, F: FnOnce(&mut AString) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.topic,
            Msg::Subscribe(msg) => &mut msg.topic,
        })
    }
}
impl Lens<Msg, QoS> for MsgTopicLens {
    fn with<V, F: FnOnce(&QoS) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.qos,
            Msg::Subscribe(msg) => &msg.qos,
        })
    }

    fn with_mut<V, F: FnOnce(&mut QoS) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.qos,
            Msg::Subscribe(msg) => &mut msg.qos,
        })
    }
}
pub struct MsgQosLens;
impl Lens<Msg, String> for MsgQosLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &Msg, f: F) -> V {
        let qos = match data {
            Msg::Public(msg) => msg.qos.to_string(),
            Msg::Subscribe(msg) => msg.qos.to_string(),
        };
        f(&qos)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut Msg, f: F) -> V {
        let mut qos = match data {
            Msg::Public(msg) => msg.qos.to_string(),
            Msg::Subscribe(msg) => msg.qos.to_string(),
        };
        f(&mut qos)
    }
}

pub struct PortLens;

impl Lens<Broker, u16> for PortLens {
    fn with<V, F: FnOnce(&u16) -> V>(&self, data: &Broker, f: F) -> V {
        f(&data.port)
    }

    fn with_mut<V, F: FnOnce(&mut u16) -> V>(&self, data: &mut Broker, f: F) -> V {
        f(&mut data.port)
    }
}

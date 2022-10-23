use crate::data::common::{Broker, SubscribeHis};
use crate::data::hierarchy::AppData;
use crate::data::lens::{BrokerStoredList, LensSelectedSubscribeHis};
use crate::data::AppEvent;
use crate::ui::common::{label_dy, label_dy_expand_width, QOS, SILVER, TOPIC};
use crate::ui::icons::{added_icon, connect_icon, copy_icon, modified_icon, removed_icon};
use druid::im::Vector;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::Svg;
use druid::widget::{
    Button, Container, CrossAxisAlignment, Either, Flex, Label, List, Padding, Scroll, Split,
};
use druid::{Env, EventCtx, UnitPoint};
use druid::{Widget, WidgetExt};
use log::error;
use std::sync::mpsc::Sender;

pub fn init_broker_list(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Padding::new(
        5.0,
        Container::new(
            Split::rows(init_connect(tx.clone()), init_subscribe_his_list(tx))
                .split_point(0.55)
                .draggable(true)
                .bar_size(3.0),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    )
}

fn init_subscribe_his_list(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let his_fn = move || {
        let tx_click = tx.clone();
        Flex::row()
            .with_child(QOS().lens(SubscribeHis::qos))
            .with_child(TOPIC().lens(SubscribeHis::topic))
            .on_click(move |_ctx, data: &mut SubscribeHis, _env| {
                if let Err(_) = tx_click.send(AppEvent::ClickSubscribeHis(data.clone())) {
                    error!("fail to send event")
                }
            })
    };

    let list: List<SubscribeHis> = List::new(move || {
        Either::new(
            |data: &SubscribeHis, _env| data.selected,
            his_fn().background(SILVER),
            his_fn(),
        )
    });
    let scroll = Scroll::<Vector<SubscribeHis>, List<SubscribeHis>>::new(list)
        .vertical()
        .lens(LensSelectedSubscribeHis);
    let buttons = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            Svg::new(removed_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::RemoveSubscribeHis) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            Svg::new(connect_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Some(his) = data.get_selected_subscribe_his() {
                    if let Err(_) = data.db.tx.send(AppEvent::SubscribeFromHis(his)) {
                        error!("fail to send event");
                    }
                }
            }),
        );

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(buttons).with_flex_child(scroll, 1.0);
    flex
}

pub fn init_connect(_tx: Sender<AppEvent>) -> Flex<AppData> {
    let name = || label_dy(|data: &Broker, _: &Env| format!("{}", data.name));
    let addr =
        || label_dy_expand_width(|data: &Broker, _: &Env| format!("{}:{}", data.addr, data.port));

    let list: List<Broker> = List::new(move || {
        Either::new(
            |data: &Broker, _env| data.selected,
            Flex::row()
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(_e) = data.tx.send(AppEvent::ClickBroker(data.id)) {
                        error!("fail to send");
                    }
                })
                .background(SILVER),
            Flex::row()
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(_e) = data.tx.send(AppEvent::ClickBroker(data.id)) {
                        error!("fail to send");
                    }
                }),
        )
    });
    let scroll = Scroll::<Vector<Broker>, List<Broker>>::new(list);

    let buttons = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            Svg::new(added_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::AddBroker) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            Svg::new(modified_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::EditBroker) {
                    error!("fail to send event")
                }
            }),
        )
        // .with_child(
        //     Svg::new(copy_icons()).on_click(move |_ctx, data: &mut AppData, _env| {
        //         if let Err(_) = data.db.tx.send(AppEvent::DeleteBroker) {
        //             error!("fail to send event")
        //         }
        //     }),
        // )
        .with_child(
            Svg::new(removed_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::DeleteBroker) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            Svg::new(connect_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::ConnectBroker) {
                    error!("fail to send event")
                }
            }),
        );

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex
        .with_child(buttons)
        .with_flex_child(scroll.vertical().expand().lens(BrokerStoredList), 1.0);
    flex
}

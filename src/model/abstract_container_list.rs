use gtk::gio;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use crate::model;

mod imp {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub(crate) struct AbstractContainerList(glib::gobject_ffi::GTypeInterface);

    #[glib::object_interface]
    unsafe impl ObjectInterface for AbstractContainerList {
        const NAME: &'static str = "AbstractContainerList";
        type Prerequisites = (gio::ListModel,);

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder(
                        "container-added",
                        &[model::Container::static_type().into()],
                        <()>::static_type().into(),
                    )
                    .build(),
                    Signal::builder(
                        "container-name-changed",
                        &[model::Container::static_type().into()],
                        <()>::static_type().into(),
                    )
                    .build(),
                    Signal::builder(
                        "container-removed",
                        &[model::Container::static_type().into()],
                        <()>::static_type().into(),
                    )
                    .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecUInt::new(
                        "len",
                        "Len",
                        "The length of this list",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "created",
                        "Created",
                        "The number of created containers",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "dead",
                        "Dead",
                        "The number of dead containers",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "exited",
                        "Exited",
                        "The number of exited containers",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "paused",
                        "Paused",
                        "The number of paused containers",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "removing",
                        "Removing",
                        "The number of containers being removed",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "running",
                        "Running",
                        "The number of running containers",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "stopped",
                        "Stopped",
                        "The number of stopped containers",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                    glib::ParamSpecUInt::new(
                        "stopping",
                        "Stopping",
                        "The number of containers being stopped",
                        0,
                        std::u32::MAX,
                        0,
                        glib::ParamFlags::READABLE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }
    }
}

glib::wrapper! {
    pub(crate) struct AbstractContainerList(ObjectInterface<imp::AbstractContainerList>)
        @requires gio::ListModel;
}

impl AbstractContainerList {
    pub(super) fn bootstrap<L: IsA<Self> + IsA<gio::ListModel>>(list: &L)
    where
        L: glib::clone::Downgrade,
        <L as glib::clone::Downgrade>::Weak: glib::clone::Upgrade,
        <<L as glib::clone::Downgrade>::Weak as glib::clone::Upgrade>::Strong: glib::IsA<Self>,
    {
        list.connect_items_changed(|self_, _, _, _| self_.notify("len"));

        list.connect_container_added(|list, container| {
            Self::notify_num_containers(list);

            container.connect_notify_local(
                Some("status"),
                clone!(@weak list => move |_, _| Self::notify_num_containers(&list)),
            );

            container.connect_notify_local(
                Some("name"),
                clone!(@weak list => move |container, _| {
                    list.container_name_changed(container);
                }),
            );
        });

        list.connect_container_removed(|list, _| Self::notify_num_containers(list));
    }

    fn notify_num_containers(list: &impl IsA<Self>) {
        list.notify("created");
        list.notify("dead");
        list.notify("exited");
        list.notify("paused");
        list.notify("removing");
        list.notify("running");
        list.notify("stopped");
        list.notify("stopping");
    }
}

pub(crate) trait AbstractContainerListExt: IsA<AbstractContainerList> {
    fn container_added(&self, container: &model::Container) {
        self.emit_by_name::<()>("container-added", &[container]);
    }

    fn container_name_changed(&self, container: &model::Container) {
        self.emit_by_name::<()>("container-name-changed", &[container]);
    }

    fn container_removed(&self, model: &model::Container) {
        self.emit_by_name::<()>("container-removed", &[&model]);
    }

    fn connect_container_added<F: Fn(&Self, &model::Container) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("container-added", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let container = values[1].get::<model::Container>().unwrap();
            f(&obj, &container);

            None
        })
    }

    fn connect_container_name_changed<F: Fn(&Self, &model::Container) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("container-name-changed", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let container = values[1].get::<model::Container>().unwrap();
            f(&obj, &container);

            None
        })
    }

    fn connect_container_removed<F: Fn(&Self, &model::Container) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("container-removed", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let container = values[1].get::<model::Container>().unwrap();
            f(&obj, &container);

            None
        })
    }
}

impl<T: IsA<AbstractContainerList>> AbstractContainerListExt for T {}

unsafe impl<T: ObjectSubclass> IsImplementable<T> for AbstractContainerList {}

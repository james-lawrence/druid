
#[derive(Debug, Clone)]
pub struct Output {
    wl_output: wl::Main<WlOutput>,
    wl_proxy: wl::Proxy<WlOutput>,
    /// global id of surface.
    pub gid: u32,
    pub x: i32,
    pub y: i32,
    pub physical_width: i32,
    pub physical_height: i32,
    pub subpixel: Subpixel,
    pub make: String,
    pub model: String,
    pub transform: Transform,
    pub scale: i32,
    pub current_mode: Option<Mode>,
    pub preferred_mode: Option<Mode>,
    /// Whether we have received some update events but not the `done` event.
    update_in_progress: bool,
    /// Lets us work out if things have changed since we last observed the output.
    last_update: Instant,
}

#[allow(unused)]
impl Output {
    // All the stuff before `current_mode` will be filled out immediately after creation, so these
    // dummy values will never be observed.
    fn new(id: u32, wl_output: wl::Main<WlOutput>) -> Self {
        Output {
            wl_output: wl_output.clone(),
            wl_proxy: wl::Proxy::from(wl_output.detach()),
            gid: id,
            x: 0,
            y: 0,
            physical_width: 0,
            physical_height: 0,
            subpixel: Subpixel::Unknown,
            make: "".into(),
            model: "".into(),
            transform: Transform::Normal,
            current_mode: None,
            preferred_mode: None,
            scale: 1, // the spec says if there is no scale event, assume 1.
            update_in_progress: true,
            last_update: Instant::now(),
        }
    }

    /// Get the wayland object ID for this output. This is how we key outputs in our global
    /// registry.
    pub fn id(&self) -> u32 {
        self.wl_proxy.id()
    }

    /// Incorporate update data from the server for this output.
    fn process_event(&mut self, evt: wl_output::Event, tx: &calloop::channel::Sender<Self>) {
        tracing::debug!("processing wayland output event {:?}", evt);
        match evt {
            wl_output::Event::Geometry {
                x,
                y,
                physical_width,
                physical_height,
                subpixel,
                make,
                model,
                transform,
            } => {
                self.x = x;
                self.y = y;
                self.physical_width = physical_width;
                self.physical_height = physical_height;
                self.subpixel = subpixel;
                self.make = make;
                self.model = model;
                self.transform = transform;
                self.update_in_progress = true;
            }
            wl_output::Event::Mode {
                flags,
                width,
                height,
                refresh,
            } => {
                if flags.contains(wl_output::Mode::Current) {
                    self.current_mode = Some(Mode {
                        width,
                        height,
                        refresh,
                    });
                }
                if flags.contains(wl_output::Mode::Preferred) {
                    self.preferred_mode = Some(Mode {
                        width,
                        height,
                        refresh,
                    });
                }
                self.update_in_progress = true;
            }
            wl_output::Event::Done => {
                self.update_in_progress = false;
                self.last_update = Instant::now();
                if let Err(cause) = tx.send(self.clone()) {
                    tracing::error!("unable to add output {:?} {:?}", self.gid, self.id());
                }
            }
            wl_output::Event::Scale { factor } => {
                self.scale = factor;
                self.update_in_progress = true;
            }
            _ => tracing::warn!("unknown output event {:?}", evt), // ignore possible future events
        }
    }

    /// Whether the output has changed since `since`.
    ///
    /// Will return `false` if an update is in progress, as updates should be handled atomically.
    fn changed(&self, since: Instant) -> bool {
        !self.update_in_progress && since < self.last_update
    }
}

#[derive(Debug, Clone)]
pub struct Mode {
    pub width: i32,
    pub height: i32,
    pub refresh: i32,
}
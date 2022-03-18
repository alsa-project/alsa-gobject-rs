// SPDX-License-Identifier: MIT
use alsaseq::{traits::*, *};
use glib::{source, translate::*, Error, MainLoop};
use nix::sys::signal;
use std::sync::Arc;

fn prepare_client(name: &str) -> Result<(UserClient, ClientInfo), Error> {
    let client = UserClient::new();
    if client.open(0).is_err() {
        eprintln!("Fail to open ALSA Sequencer character device.");
        std::process::exit(1);
    }

    let mut info = ClientInfo::new();
    if client.get_info(&mut info).is_err() {
        eprintln!("Fail to get the information of client.");
        std::process::exit(1);
    }

    info.set_name(Some(name));
    if client.set_info(&mut info).is_err() {
        eprintln!("Fail to set the information of clinent.");
        std::process::exit(1);
    }

    Ok((client, info))
}

fn prepare_port(client: &UserClient, name: &str) -> Result<PortInfo, Error> {
    let mut info = PortInfo::new();

    info.set_name(Some(name));

    let caps = PortCapFlag::READ
        | PortCapFlag::WRITE
        | PortCapFlag::DUPLEX
        | PortCapFlag::SUBS_READ
        | PortCapFlag::SUBS_WRITE;
    info.set_caps(caps);

    let attrs = PortAttrFlag::MIDI_GENERIC | PortAttrFlag::SOFTWARE | PortAttrFlag::APPLICATION;
    info.set_attrs(attrs);

    client.create_port(&mut info)?;

    Ok(info)
}

fn prepare_queue(client: &UserClient, port: &PortInfo, name: &str) -> Result<QueueInfo, Error> {
    let mut info = QueueInfo::new();

    info.set_name(Some(name));
    info.set_locked(true);

    client.create_queue(&mut info)?;

    let ev_cntr = EventCntr::new(1)?;
    ev_cntr.set_event_type(0, EventType::Start)?;
    ev_cntr.set_tstamp_mode(0, EventTimestampMode::Real)?;
    ev_cntr.set_time_mode(0, EventTimeMode::Rel)?;
    ev_cntr.set_priority_mode(0, EventPriorityMode::Normal)?;
    ev_cntr.set_tag(0, 0)?;
    ev_cntr.set_queue_id(0, SpecificQueueId::Direct.into_glib() as u8)?;
    let addr = Addr::new(
        SpecificClientId::System.into_glib() as u8,
        SpecificPortId::Timer.into_glib() as u8,
    );
    ev_cntr.set_dst(0, &addr)?;
    if let Some(addr) = port.addr() {
        ev_cntr.set_src(0, &addr)?;
    }
    let mut data = ev_cntr.get_queue_data(0)?;
    data.set_queue_id(info.queue_id() as u8);
    ev_cntr.set_queue_data(0, &data)?;

    client.schedule_event(&ev_cntr, 1)?;

    Ok(info)
}

fn dump_info(client: &ClientInfo, port: &PortInfo, queue: &QueueInfo) {
    println!("Client: {}", client.name().expect(""));
    println!("  card-id:                {}", client.card_id());
    println!("  client-id:              {}", client.client_id());
    println!("  filter-attrs:           {:?}", client.filter_attributes());
    println!("  lost-count:             {}", client.lost_count());
    println!("  port-count:             {}", client.port_count());
    println!("  process-id:             {}", client.process_id());
    println!("  type:                   {}", client.type_());
    println!("  use-filter:             {}", client.uses_filter());

    println!("Port: {}", port.name().expect(""));
    if let Some(addr) = port.addr() {
        println!("  client:                 {}", addr.client_id());
        println!("  port:                   {}", addr.port_id());
    }
    println!("  attrs:                  {:?}", port.attrs());
    println!("  caps:                   {:?}", port.caps());
    println!("  midi channels:          {}", port.midi_channels());
    println!("  midi voices:            {}", port.midi_voices());
    println!("  queue-id:               {}", port.queue_id());
    println!("  read users:             {}", port.read_users());
    println!("  synth voices:           {}", port.synth_voices());
    println!("  timestamp-mode:         {}", port.timestamp_mode());
    println!(
        "  timestamp-overwrite:    {}",
        port.is_timestamp_overwrite()
    );
    println!("  write users:            {}", port.write_users());

    println!("Queue: {}", queue.name().expect(""));
    println!("  client-id:              {}", queue.client_id());
    println!("  locked:                 {}", queue.is_locked());
    println!("  queue-id:               {}", queue.queue_id());
}

fn run_dispatcher(client: &UserClient) -> Result<(), Error> {
    let dispatcher = MainLoop::new(None, false);
    let ctx = dispatcher.context();

    let dispatcher_cntr = Arc::new(dispatcher);
    let d = dispatcher_cntr.clone();

    let src = source::unix_signal_source_new(
        signal::Signal::SIGINT as i32,
        None,
        source::PRIORITY_DEFAULT_IDLE,
        move || {
            d.quit();
            source::Continue(true)
        },
    );
    src.attach(Some(&ctx));

    let src = client.create_source()?;
    src.attach(Some(&ctx));

    client.connect_handle_event(|_, ev_cntr| {
        let count = ev_cntr.count_events();
        println!("Event count: {}", count);
        (0..count)
            .try_for_each(|i| {
                let ev_type = ev_cntr.event_type(i)?;
                let tstamp_mode = ev_cntr.tstamp_mode(i)?;
                println!("  Event {}:           {}", i, ev_type);
                println!("    length-mode:      {}", ev_cntr.length_mode(i)?);
                println!("    priority-mode:    {}", ev_cntr.priority_mode(i)?);
                println!("    time-mode:        {}", ev_cntr.time_mode(i)?);
                println!("    tstamp-mode:      {}", tstamp_mode);
                println!("    queue-id:         {}", ev_cntr.queue_id(i)?);
                println!("    tag:              {}", ev_cntr.tag(i)?);

                let tstamp = ev_cntr.get_tstamp(i)?;
                if tstamp_mode == EventTimestampMode::Tick {
                    println!("    tick-time:        {}", tstamp.get_tick_time());
                } else {
                    let real_time = tstamp.get_real_time();
                    println!("    real-time:        {}.{}", real_time[0], real_time[1]);
                }

                let src = ev_cntr.get_src(i)?;
                println!("    src:");
                println!("      client-id:      {}", src.client_id());
                println!("      port-id:        {}", src.port_id());

                let dst = ev_cntr.get_dst(i)?;
                println!("    dst:");
                println!("      client-id:      {}", dst.client_id());
                println!("      port-id:        {}", dst.port_id());

                match ev_type {
                    EventType::Note
                    | EventType::Noteon
                    | EventType::Noteoff
                    | EventType::Keypress => {
                        let data = ev_cntr.note_data(i)?;
                        println!("    note data:");
                        println!("      channel:        {}", data.channel());
                        println!("      note:           {}", data.note());
                        println!("      duration:       {}", data.duration());
                        println!("      velocity:       {}", data.velocity());
                        println!("      off-velocity:   {}", data.off_velocity());
                    }
                    EventType::Pgmchange
                    | EventType::Chanpress
                    | EventType::Pitchbend
                    | EventType::Control14
                    | EventType::Nonregparam
                    | EventType::Regparam
                    | EventType::Songpos
                    | EventType::Songsel
                    | EventType::Qframe
                    | EventType::Timesign
                    | EventType::Keysign => {
                        let data = ev_cntr.get_ctl_data(i)?;
                        println!("    ctl data:");
                        println!("      channel:        {}", data.channel());
                        println!("      param:          {}", data.param());
                        println!("      value:          {}", data.value());
                    }
                    _ => (),
                }

                Ok::<(), Error>(())
            })
            .unwrap();
    });

    dispatcher_cntr.run();

    Ok(())
}

fn main() {
    match prepare_client("focal") {
        Err(_) => eprintln!("Fail to prepare user client."),
        Ok((client, client_info)) => match prepare_port(&client, "fossa") {
            Err(_) => eprintln!("Fail to prepare port for the user client."),
            Ok(port_info) => {
                match prepare_queue(&client, &port_info, "20.04") {
                    Err(_) => eprintln!("Fail to prepare port for the user client."),
                    Ok(queue_info) => {
                        dump_info(&client_info, &port_info, &queue_info);

                        run_dispatcher(&client).unwrap();

                        let queue_id = queue_info.queue_id();
                        client.delete_queue(queue_id).unwrap();
                    }
                }
                if let Some(addr) = port_info.addr() {
                    let port_id = addr.port_id();
                    client.delete_port(port_id).unwrap();
                }
            }
        },
    }

    std::process::exit(0);
}

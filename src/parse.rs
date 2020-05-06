use nom::number::streaming::be_u8;
use crate::command::Command;

// get socket path with i3 --get-socketpath

/*
 * So, a typical message could look like this:
 * --------------------------------------------------
 * "i3-ipc" <message length> <message type> <payload>
 * --------------------------------------------------
 */
#[derive(Debug, PartialEq)]
pub struct Ipc {
    length: u8,
    tpe: Command,
    payload: Vec<u8>
}

named!(
    pub ipc<Ipc>,
    do_parse!(
        tag!("i3-ipc") >>
        length: be_u8  >>
        tpe: switch!(
                be_u8,
                1 => value!(Command::RunCommand) |
                2 => value!(Command::GetWorkspaces) |
                3 => value!(Command::Subscribe) |
                4 => value!(Command::GetOutputs) |
                5 => value!(Command::GetTree) |
                6 => value!(Command::GetMarks) |
                7 => value!(Command::GetBarConfig) |
                8 => value!(Command::GetBiningModes) |
                9 => value!(Command::GetVersion) |
                10 => value!(Command::SendTick) |
                11 => value!(Command::Sync)
            )  >>
        payload: many_m_n!(length.into(), length.into(), be_u8) >>
        (Ipc {
            length: length,
            tpe: tpe,
            payload: payload
        })
    )
);


#[test]
fn test_ipc_run_command_works() {
    assert_eq!(
        ipc(&hex!("69 33 2d 69 70 63 02 01 44 55")),
        Ok((&[] as &[u8], Ipc { length: 2, tpe: Command::RunCommand, payload: vec!(68, 85) }))
    )
}

#[test]
fn test_ipc_get_workspaces_works() {
    assert_eq!(
        ipc(&hex!("69 33 2d 69 70 63 00 02")),
        Ok((&[] as &[u8], Ipc { length: 0, tpe: Command::GetWorkspaces, payload: vec!() }))
    )
}

#[test]
fn test_no_magic_word_fails() {
    let input: &[u8] = &hex!("00 02");

    assert_eq!(
        ipc(input),
        Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)))
    )
}

/**
 *
 * | type (numeric) | type (name)         | reply type                             | purpose                                                                                            |
 * |----------------|---------------------|----------------------------------------|----------------------------------------------------------------------------------------------------|
 * | 0              | +RUN_COMMAND+       | <<_command_reply,COMMAND>>             | Run the payload as an i3 command (like the commands you can bind to keys).                         |
 * | 1              | +GET_WORKSPACES+    | <<_workspaces_reply,WORKSPACES>>       | Get the list of current workspaces.                                                                |
 * | 2              | +SUBSCRIBE+         | <<_subscribe_reply,SUBSCRIBE>>         | Subscribe this IPC connection to the event types specified in the message payload. See <<events>>. |
 * | 3              | +GET_OUTPUTS+       | <<_outputs_reply,OUTPUTS>>             | Get the list of current outputs.                                                                   |
 * | 4              | +GET_TREE+          | <<_tree_reply,TREE>>                   | Get the i3 layout tree.                                                                            |
 * | 5              | +GET_MARKS+         | <<_marks_reply,MARKS>>                 | Gets the names of all currently set marks.                                                         |
 * | 6              | +GET_BAR_CONFIG+    | <<_bar_config_reply,BAR_CONFIG>>       | Gets the specified bar configuration or the names of all bar configurations if payload is empty.   |
 * | 7              | +GET_VERSION+       | <<_version_reply,VERSION>>             | Gets the i3 version.                                                                               |
 * | 8              | +GET_BINDING_MODES+ | <<_binding_modes_reply,BINDING_MODES>> | Gets the names of all currently configured binding modes.                                          |
 * | 9              | +GET_CONFIG+        | <<_config_reply,CONFIG>>               | Returns the last loaded i3 config.                                                                 |
 * | 10             | +SEND_TICK+         | <<_tick_reply,TICK>>                   | Sends a tick event with the specified payload.                                                     |
 * | 11             | +SYNC+              | <<_sync_reply,SYNC>>                   | Sends an i3 sync event with the specified random value to the specified window.                    |
 */
#[derive(Debug, PartialEq)]
pub enum Command {
    RunCommand,
    GetWorkspaces,
    Subscribe,
    GetOutputs,
    GetTree,
    GetMarks,
    GetBarConfig,
    GetBiningModes,
    GetVersion,
    SendTick,
    Sync
}

#[derive(Debug, PartialEq)]
pub struct UnknownCommand {
    pub msg : String
}

pub fn parse_command(cmd: String) -> Result<Command, UnknownCommand> {
    match cmd.as_str() {
        "RunCommand" => Ok(Command::RunCommand),
        "GetWorkspaces" => Ok(Command::GetWorkspaces),
        "Subscribe" => Ok(Command::Subscribe),
        "GetOutputs" => Ok(Command::GetOutputs),
        "GetTree" => Ok(Command::GetTree),
        "GetMarks" => Ok(Command::GetMarks),
        "GetBarConfig" => Ok(Command::GetBarConfig),
        "GetBiningModes" => Ok(Command::GetBiningModes),
        "GetVersion" => Ok(Command::GetVersion),
        "SendTick" => Ok(Command::SendTick),
        "Sync" => Ok(Command::Sync),
        other => Err(UnknownCommand { msg: other.to_string() })
    }
}

#[test]
fn test_parse_command() {
    assert_eq!(
        parse_command("RunCommand".to_string()),
        Ok(Command::RunCommand)
    )
}

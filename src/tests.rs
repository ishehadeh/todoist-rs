use super::*;
use command::Create;
#[test]
fn sanity() {
    assert_eq!(4, 2 + 2);
}


#[test]
fn project_create() {
    let mut new_project = resource::Project::default();
    new_project.name = "my cool project".to_string();
    new_project.is_favorite = IntBool::from(true);
    
    let cmd = new_project.create();
    let cmd_str = serde_json::to_string(&cmd).unwrap();
    assert_eq!(cmd_str,
               format!("{{\"type\":\"project_add\",\
                      \"args\":\
                      {{\"name\":\"my cool project\",\
                        \"color\":0,\"indent\":0,\
                        \"item_order\":0,\"is_favorite\":1}},\
                      \"uuid\":\"{}\",\
                      \"temp_id\":\"{}\"}}", cmd.uuid, cmd.temp_id.unwrap()));
}
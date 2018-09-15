use uuid::Uuid;
use types::ID;
#[macro_use] mod macros;

pub mod filter {
    use types::{Color, ID};
    
    command! {
        pub struct Add {
            name  : String,
            query : String,
            color : Color,
            item_order  : isize,
            is_favorite : isize
        }
    }

    command! {
        pub struct Update {
            id          : ID,
            name        : String,
            query       : String,
            color       : Color,
            item_order  : isize,
            is_favorite : isize
        }
    }

    identity_list_command!(Delete);
}

pub mod item {
    use types::*;
    use std::collections::HashMap;

    command! {
        pub struct Add {
             project_id : ID,
             content : Option<String>,
             date_string : Option<String>,
             date_lang : Option<Language>,
             due_date_utc : Option<Date>,
             priority : Priority,
             indent : u8,
             item_order : isize,
             day_order : isize,
             collapsed : isize,
             labels : Vec<ID>,
             assigned_by_uid : Option<ID>,
             auto_reminders : Option<bool>,
             auto_parse_labels : Option<bool>
        }
    }

    command! {
        pub struct Update {
             id : ID,
             content : Option<String>,
             date_string : Option<String>,
             date_lang : Option<Language>,
             due_date_utc : Option<Date>,
             priority : Priority,
             indent : u8,
             item_order : isize,
             day_order : isize,
             collapsed : isize,
             labels : Vec<ID>,
             assigned_by_uid : Option<ID>,
             responsible_uid : Option<ID>
        }
    }


    command! {
        pub struct Move {
             project_items : HashMap<ID, ID>,
             to_project    : ID
        }
    }

    identity_list_command!(Archive);
    identity_list_command!(Unarchive);
    identity_list_command!(Close);
    identity_list_command!(Delete);
}

pub mod label {
    use types::*;

    command! {
        pub struct Add {
            name : String,
            color : Color,
            item_order : isize,
            is_favorite : isize
        }
    }

    command! {
        pub struct Update {
            id   : ID,
            name : String,
            color : Color,
            item_order : isize,
            is_favorite : isize
        }
    }

    identity_list_command!(Delete);

}

pub mod note {
    use types::*;
    use resource::Attachment;

    command! {
        pub struct Add {
            item_id         : Option<ID>,
            project_id      : Option<ID>,
            content         : String,
            file_attachment : Option<Attachment>,
            uids_to_notify  : Option<Vec<ID> >
        }
    }

    command! {
        pub struct Update {
            id : ID,
            content         : String,
            file_attachment : Option<Attachment>
        }
    }


    identity_list_command!(Delete);
}

pub mod project {
    use types::*;

    command! {
        pub struct Add {
            name        : String,
            color       : Color,
            indent      : u8,
            item_order  : isize,
            is_favorite : isize
        }
    }

    command! {
        pub struct Update {
            id          : ID,
            name        : String,
            color       : Color,
            indent      : u8,
            item_order  : isize,
            collapsed   : isize,
            is_favorite : isize
        }
    }

    identity_list_command!(Delete);
}

make_argument_enum! {
    pub enum CommandArgs {
        FilterAdd(filter::Add),
        FilterUpdate(filter::Update),
        FilterDelete(filter::Delete),

        ItemAdd(item::Add),
        ItemUpdate(item::Update),
        ItemMove(item::Move),
        ItemArchive(item::Archive),
        ItemUnarchive(item::Unarchive),
        ItemClose(item::Close),

        LabelAdd(label::Add),
        LabelUpdate(label::Update),
        LabelDelete(label::Delete),

        NoteAdd(note::Add),
        NoteUpdate(note::Update),
        NoteDelete(note::Delete),

        ProjectAdd(project::Add),
        ProjectUpdate(project::Update),
        ProjectDelete(project::Delete)
    }
}

#[derive(Serialize)]
pub struct Command {
    #[serde(flatten)]
    pub args    : CommandArgs,
    pub uuid    : Uuid,
    pub temp_id : Option<Uuid>, 
}
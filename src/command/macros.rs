
macro_rules! command_arguments {
    (Vec<ID>, $field:ident, $($typs:tt $(<$($generics:tt $(<$generics2:tt>)*),+>)*, $fields:ident),*) => {
        pub fn $field<T : Into<ID>>(mut self, a : T) -> Self {
            self.$field.push(a.into());
            self
        }
        command_arguments!($($typs $(<$($generics$(<$generics2>)* ),+>)*, $fields),*);
    };

    (Option<$typ:tt $(<$generic:tt>)*>, $field:ident, $($typs:tt $(<$($generics:tt $(<$generics2:tt>)*),+>)*, $fields:ident),*) => {
        pub fn $field(mut self, a : $typ) -> Self {
            self.$field = Some(a);
            self
        }
        command_arguments!($($typs $(<$($generics$(<$generics2>)* ),+>)*, $fields),*);
    };

    (ID, $field:ident, $($typs:tt $(<$($generics:tt $(<$generics2:tt>)*),+>)*, $fields:ident),*) => {
        pub fn $field<T : Into<ID>>(mut self, a : T) -> Self {
            self.$field = a.into();
            self
        }
        command_arguments!($($typs $(<$($generics$(<$generics2>)* ),+>)*, $fields),*);
    };

    (String, $field:ident, $($typs:tt $(<$($generics:tt $(<$generics2:tt>)*),+>)*, $fields:ident),*) => {
        pub fn $field<T : AsRef<str>>(mut self, a : T) -> Self {
            self.$field = a.as_ref().to_owned();
            self
        }
        command_arguments!($($typs $(<$($generics$(<$generics2>)* ),+>)*, $fields),*);
    };

    ($typ:tt $(<$($generic:tt $(<$generic2:tt>)*),+>)*, $field:ident, $($typs:tt $(<$($generics:tt $(<$generics2:tt>)*),+>)*, $fields:ident),*) => {
        pub fn $field(mut self, s : $typ $(<$($generic$(<$generic2>)* ),+>)*) -> Self {
            self.$field = s;
            self
        }
        command_arguments!($($typs $(<$($generics$(<$generics2>)* ),+>)*, $fields),*);
    };

    ($typ:tt $(<$($generic:tt $(<$generic2:tt>)*),+>)*, $field:ident, $($typs:tt $(<$($generics:tt $(<$generics2:tt>)*),+>)*, $fields:ident),*) => {
        pub fn $field(mut self, s : $typ $(<$($generic$(<$generic2>)* ),+>)*) -> Self {
            self.$field = s;
            self
        }
        command_arguments!($($typs $(<$($generics$(<$generics2>)* ),+>)*, $fields),*);
    };

    ($typ:tt $(<$($generic:tt $(<$generic2:tt>)*),+>)*, $field:ident) => {
        pub fn $field(mut self, s : $typ $(<$($generic$(<$generic2>)* ),+>)*) -> Self {
            self.$field = s;
            self
        }
    };
}

macro_rules! command {
    (pub struct $name:ident { $($field:ident : $typ:tt $(<$($generic:tt $(<$generic2:tt>)*),+>)* ),+ }) => {
        #[derive(Serialize, Deserialize, Default, Debug)]
        #[serde(default)]
        pub struct $name {
            $(
                pub $field : $typ $(<$($generic$(<$generic2>)* ),+>)*,
            )*
        }

        impl $name {
            command_arguments!($($typ $(<$($generic$(<$generic2>)* ),+>)*, $field),*);
        }
    };
}

macro_rules! identity_list_command {
    ($name:ident) => (
        command! {
            pub struct $name {
                    ids: Vec<ID>
            }
        }
    );
}

macro_rules! make_argument_enum {
    (pub enum $name:ident { $($field:ident($typ:path)),* }) => {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(tag = "type", content = "args", rename_all = "snake_case")]
        pub enum $name {
            $($field($typ)),*
        }

        $(
            impl From<$typ> for $name {
                fn from(t : $typ) -> Self {
                    $name::$field(t)
                }
            }
        )*
    }
}
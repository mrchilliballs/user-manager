#[macro_export]
macro_rules! optionalize {
    (
    #[new_name = $new_struct_name:ident]
    #[$struct_attr:meta]
    $visibility:vis struct $struct_name:ident {
        $($field_vis:vis $field_name:ident : $field_type:ty),* $(,)?
    }
    ) => {
        #[$struct_attr]
        $visibility struct $struct_name {
            $($field_vis $field_name : $field_type),*
        }
        #[derive(Debug, PartialEq, Clone, Args)]
        $visibility struct $new_struct_name {
            $($field_vis $field_name : Option<$field_type>),*
        }
        // Rename this terrible function
        impl $new_struct_name {
            // Clones user
            pub fn to_original(self, value: &$struct_name) -> $struct_name {
                    $(
                        let $field_name: $field_type;
                        if let Some(val) = self.$field_name {
                            $field_name = val;
                        } else  {
                            $field_name = value.$field_name.clone();
                        }
                    )+

                $struct_name {
                    $(
                        $field_name
                    ),+
                }
            }
        }
    }
}
macro_rules! generate_function_name {
    () => {};
}

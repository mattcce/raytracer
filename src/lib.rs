pub mod collections;
pub mod objects;
pub mod scenes;
pub(crate) mod utils;

// public interface re-exports (import with use raytracer::prelude::*)
pub mod prelude {
    pub use super::collections::prelude::*;
    pub use super::objects::prelude::*;
    pub use super::scenes::prelude::*;
}

/*
    # Re-exports and `use` organisation

    All 'mod.rs' files should only contain re-exports and module declarations.

    All non-terminal modules must have preludes.

    Two types of re-exports are commonly used: Crate-level re-exports: re-
    - exports used within the crate; every name is exposed Public re-exports:
    - re-exports in the public interface; only commonly used names are exposed

    ## Public re-exports

    For public re-exports, all exporting is done through flattening preludes.
    Each module, including the top-level module, will import all names in the
    preludes of the immediate nested modules using the glob operator. These
    names should be commonly used to be included in the prelude.

    In general, each non-top-level module with submodules should import the
    submodule preludes or names, or the submodule itself (exposing the submodule
    name directly) using
        ```
        pub(super) mod prelude {
            // for submodules as a name (as-is)
            pub use super::submodule;
            // for names in non-terminal submodules with preludes
            pub use super::submodule::prelude::*;
            // for names in terminal submodules without preludes
            pub use super::submodule::{Name1, Name2};
        }
        ```

    The top-level module prelude is the public interface prelude that imports
    all of the names in each submodule's prelude.


    ## Crate-level re-exports

    For crate-level re-exports, all names are re-exported to de-nest exactly
    one level of nesting. This only applies to modules containing terminal
    submodules (that is, submodules that do not have sub-submodules).

    In general, a name like crate::module::submodule::Name should be re-
    exported as crate::module::Name in src/module/mod.rs.

    In src/module/mod.rs, bring nested names into scope using
        `pub(crate) use submodule::Name`.

    Submodules within the same parent module should bring names into scope using
        `use super::Name`.

    Modules outside of this parent module should bring names into scope using
        `use crate::module::Name`.

    Test modules should always bring names that only it needs into scope using
        `use crate::...`
    to avoid having to write `use super::super::...`.
*/

mod world;

use godot::prelude::*;

struct VoxelExtension;

#[gdextension]
unsafe impl ExtensionLibrary for VoxelExtension {}

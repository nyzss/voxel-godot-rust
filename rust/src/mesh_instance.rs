use godot::{
    classes::{
        ArrayMesh, IMeshInstance3D, MeshInstance3D,
        mesh::{self, ArrayType},
    },
    obj::IndexEnum,
    prelude::*,
};

#[derive(Debug, GodotConvert, Var, Export)]
#[godot(via = GString)]
enum Face {
    BOTTOM,
    FRONT,
    RIGHT,
    TOP,
    LEFT,
    BACK,
}

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct MeshInstance {
    base: Base<MeshInstance3D>,

    mesh: OnReady<Gd<ArrayMesh>>,
    surface_array: VarArray,

    vertices: PackedVector3Array,
    normals: PackedVector3Array,
    colors: PackedColorArray,

    cube_vertices: [Vector3; 8],
    face_indices: [[[usize; 3]; 2]; 6],
    face_normals: [Vector3; 6],
    face_colors: [Color; 6],
}

#[godot_api]
impl MeshInstance {
    #[func]
    fn generate_mesh(&mut self) {
        self.add_face(Face::BOTTOM, Vector3::ZERO);
        self.add_face(Face::FRONT, Vector3::ZERO);
        self.add_face(Face::RIGHT, Vector3::ZERO);
        self.add_face(Face::TOP, Vector3::ZERO);
        self.add_face(Face::LEFT, Vector3::ZERO);
        self.add_face(Face::BACK, Vector3::ZERO);

        self.commit_mesh();
    }

    #[func]
    fn commit_mesh(&mut self) {
        self.surface_array
            .set(ArrayType::VERTEX.to_index(), &self.vertices.to_variant());

        self.surface_array
            .set(ArrayType::NORMAL.to_index(), &self.normals.to_variant());

        self.surface_array
            .set(ArrayType::COLOR.to_index(), &self.colors.to_variant());

        self.mesh
            .add_surface_from_arrays(mesh::PrimitiveType::TRIANGLES, &self.surface_array);
    }

    #[func]
    fn add_face(&mut self, face: Face, position: Vector3) {
        let face_index = face as usize;
        let indices = self.face_indices[face_index];
        let normal = self.face_normals[face_index];
        let color = self.face_colors[face_index];

        for triangle in indices {
            for index in triangle {
                let vertex = self.cube_vertices[index];
                let pos = vertex + position;

                self.vertices.push(pos);
                self.normals.push(normal);
                self.colors.push(color);
            }
        }
    }
}

#[godot_api]
impl IMeshInstance3D for MeshInstance {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self {
            base,

            mesh: OnReady::manual(),
            surface_array: VarArray::new(),

            vertices: PackedVector3Array::new(),
            normals: PackedVector3Array::new(),
            colors: PackedColorArray::new(),

            cube_vertices: [
                Vector3::new(-0.5, -0.5, 0.5),
                Vector3::new(0.5, -0.5, 0.5),
                Vector3::new(0.5, -0.5, -0.5),
                Vector3::new(-0.5, -0.5, -0.5),
                Vector3::new(-0.5, 0.5, 0.5),
                Vector3::new(0.5, 0.5, 0.5),
                Vector3::new(0.5, 0.5, -0.5),
                Vector3::new(-0.5, 0.5, -0.5),
            ],

            face_indices: [
                [[0, 1, 2], [0, 2, 3]],
                [[0, 4, 5], [0, 5, 1]],
                [[1, 5, 6], [1, 6, 2]],
                [[4, 7, 6], [4, 6, 5]],
                [[3, 7, 4], [3, 4, 0]],
                [[2, 7, 3], [2, 6, 7]],
            ],

            face_normals: [
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
            ],

            face_colors: [
                Color::RED,
                Color::ORANGE,
                Color::YELLOW,
                Color::GREEN,
                Color::BLUE,
                Color::PURPLE,
            ],
        }
    }

    fn ready(&mut self) {
        let mesh = self
            .base_mut()
            .get_mesh()
            .unwrap()
            .try_cast::<ArrayMesh>()
            .unwrap();

        self.mesh.init(mesh);

        self.surface_array
            .resize(ArrayType::MAX.to_index(), &Variant::default());

        self.generate_mesh();
    }
}

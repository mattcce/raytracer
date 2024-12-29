use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::collections::{Point, Vector};
use crate::objects::{Group, GroupTransformable, Transform, Triangle};

type ParsedObjects = (
    Vec<Point>,
    Vec<Vector>,
    Vec<Triangle>,
    Vec<Rc<RefCell<Group>>>,
);

pub fn parse_obj(file_path: &str) -> Result<ParsedObjects, Box<dyn std::error::Error>> {
    let mut file_contents_as_string = String::new();
    File::open(file_path)?.read_to_string(&mut file_contents_as_string)?;
    let file_lines: Vec<&str> = file_contents_as_string.split("\n").collect();

    let mut parsed_vertices = vec![];
    let mut parsed_normals = vec![];
    let mut parsed_shapes: Vec<Triangle> = vec![];
    let mut parsed_groups = vec![];

    let default_group = Group::new::<Triangle>(Transform::default(), vec![]);
    let mut current_group: Option<Rc<RefCell<Group>>> = None;

    parsed_groups.push(Rc::clone(&default_group));

    for line in file_lines {
        match line.split(" ").collect::<Vec<&str>>() {
            vertex if vertex[0] == "v" => {
                if let [x_str, y_str, z_str] = vertex[1..4] {
                    let x = x_str.parse()?;
                    let y = y_str.parse()?;
                    let z = z_str.parse()?;

                    parsed_vertices.push(Point::new(x, y, z));
                } else {
                    continue;
                }
            }

            vertex_normal if vertex_normal[0] == "vn" => {
                if let [x_str, y_str, z_str] = vertex_normal[1..4] {
                    let x = x_str.parse()?;
                    let y = y_str.parse()?;
                    let z = z_str.parse()?;

                    parsed_normals.push(Vector::new(x, y, z));
                } else {
                    continue;
                }
            }

            face if face[0] == "f" => {
                if face.len() >= 4 {
                    let vertex_indices_as_str = face[1..].to_vec();

                    let mut vertices = vec![];
                    for vertex_idx_str in vertex_indices_as_str {
                        let vertex_idx: usize = vertex_idx_str.parse()?;

                        // 1-indexed to 0-indexed array indices
                        vertices.push(parsed_vertices[vertex_idx - 1]);
                    }

                    let triangles = face_triangulation(vertices);

                    for mut triangle in triangles {
                        if current_group.is_some() {
                            current_group
                                .as_mut()
                                .unwrap()
                                .borrow_mut()
                                .add_object(&mut triangle);
                        } else {
                            default_group.borrow_mut().add_object(&mut triangle);
                        }

                        parsed_shapes.push(triangle);
                    }
                } else {
                    if let [idx1_str, idx2_str, idx3_str] = face[1..4] {
                        let idx1: usize = idx1_str.parse()?;
                        let idx2: usize = idx2_str.parse()?;
                        let idx3: usize = idx3_str.parse()?;

                        // 1-indexed to 0-indexed array indices
                        let vertex1 = parsed_vertices[idx1 - 1];
                        let vertex2 = parsed_vertices[idx2 - 1];
                        let vertex3 = parsed_vertices[idx3 - 1];

                        let mut triangle = Triangle::new(vertex1, vertex2, vertex3);
                        if current_group.is_some() {
                            current_group
                                .as_mut()
                                .unwrap()
                                .borrow_mut()
                                .add_object(&mut triangle);
                        } else {
                            default_group.borrow_mut().add_object(&mut triangle);
                        }

                        parsed_shapes.push(triangle);
                    } else {
                        continue;
                    }
                }
            }

            group if group[0] == "g" => {
                if let Some(old_group) = current_group {
                    parsed_groups.push(old_group);
                }

                let new_group = Group::new::<Triangle>(Transform::default(), vec![]);
                current_group = Some(new_group);
                current_group
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .set_parent(Rc::clone(&default_group));
            }

            _ => continue,
        }
    }

    if let Some(old_group) = current_group {
        parsed_groups.push(old_group);
    }

    Ok((
        parsed_vertices,
        parsed_normals,
        parsed_shapes,
        parsed_groups,
    ))
}

fn face_triangulation(vertices: Vec<Point>) -> Vec<Triangle> {
    assert!(vertices.len() >= 3);

    let mut parsed_triangles = vec![];

    let vertex1 = vertices[0];
    for (&vertex2, &vertex3) in vertices[1..].iter().zip(vertices[2..].iter()) {
        parsed_triangles.push(Triangle::new(vertex1, vertex2, vertex3));
    }

    parsed_triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objparser_ignores_unrecognised_commands() {
        let parsed_objects = parse_obj("./resources/gibberish.obj").unwrap();
        let (parsed_vertices, parsed_normals, parsed_triangles, parsed_groups) = parsed_objects;
        assert_eq!(parsed_vertices.len(), 0);
        assert_eq!(parsed_normals.len(), 0);
        assert_eq!(parsed_triangles.len(), 0);
        assert_eq!(parsed_groups.len(), 1);
    }

    #[test]
    fn objparser_parses_vertex_data() {
        let parsed_objects = parse_obj("./resources/vertex.obj").unwrap();
        let parsed_vertices = parsed_objects.0;
        assert_eq!(parsed_vertices.len(), 4);
        assert_eq!(parsed_vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(parsed_vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(parsed_vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(parsed_vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn objparser_parses_triangle_data() {
        let parsed_objects = parse_obj("./resources/triangle.obj").unwrap();
        let parsed_shapes = parsed_objects.2;
        assert_eq!(parsed_shapes.len(), 2);
    }

    #[test]
    fn objparser_parses_polygon_data() {
        let parsed_objects = parse_obj("./resources/polygon.obj").unwrap();
        let parsed_shapes = parsed_objects.2;
        assert_eq!(parsed_shapes.len(), 3);
    }

    #[test]
    fn objparser_parses_groups() {
        let parsed_objects = parse_obj("./resources/group.obj").unwrap();
        let (_, _, _, parsed_groups) = parsed_objects;

        assert_eq!(parsed_groups.len(), 3);
    }
}

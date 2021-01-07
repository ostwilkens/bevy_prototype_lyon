//! Common shapes, like rectangles, ellipses, triangles and more.

use crate::{
    conversions::{ToLyonPoint, ToLyonVector},
    create_sprite,
    shape_plugin::ShapeDescriptor,
    Geometry, ShapeSprite, TessellationMode, Tessellator,
};
use bevy::prelude::*;
use lyon_tessellation::{
    math::{Angle, Point, Rect, Size},
    path::{Polygon, Winding},
    BuffersBuilder, FillVertex, StrokeVertex, VertexBuffers,
};

/// Defines where the origin, or pivot of the `Rectangle` should be positioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectangleOrigin {
    Center,
    BottomLeft,
    BottomRight,
    TopRight,
    TopLeft,
}

impl Default for RectangleOrigin {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangleShape {
    pub width: f32,
    pub height: f32,
    pub origin: RectangleOrigin,
}

impl Default for RectangleShape {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            origin: RectangleOrigin::default(),
        }
    }
}

impl ShapeSprite for RectangleShape {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        use RectangleOrigin::*;
        let origin = match self.origin {
            Center => Point::new(-self.width / 2.0, -self.height / 2.0),
            BottomLeft => Point::new(0.0, 0.0),
            BottomRight => Point::new(-self.width, 0.0),
            TopRight => Point::new(-self.width, -self.height),
            TopLeft => Point::new(0.0, -self.height),
        };

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_rectangle(
                        &Rect::new(origin, Size::new(self.width, self.height)),
                        &options,
                        output,
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_rectangle(
                        &Rect::new(origin, Size::new(self.width, self.height)),
                        &options,
                        output,
                    )
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleShape {
    /// Distance of the border of the circle from the center.
    pub radius: f32,
    /// The position of the center of the circle, relative to the world
    /// [`Translation`] of the [`SpriteBundle`].
    pub center: Vec2,
}

impl CircleShape {
    pub fn draw(
        &self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> (ShapeDescriptor,) {
        let desc = ShapeDescriptor {
            shape: Box::new(self.clone()),
            material: material.clone(),
            mode,
            transform,
        };

        (desc,)
    }
}

impl Default for CircleShape {
    fn default() -> Self {
        Self {
            radius: 1.0,
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for CircleShape {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_circle(self.center.to_lyon_point(), self.radius, &options, output)
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_circle(self.center.to_lyon_point(), self.radius, &options, output)
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipseShape {
    pub radii: Vec2,
    /// The position of the center of the ellipse, relative to the world
    /// [`Translation`] of the [`SpriteBundle`].
    pub center: Vec2,
}

impl Default for EllipseShape {
    fn default() -> Self {
        Self {
            radii: Vec2::one(),
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for EllipseShape {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_ellipse(
                        self.center.to_lyon_point(),
                        self.radii.to_lyon_vector(),
                        Angle::zero(),
                        Winding::Positive,
                        &options,
                        output,
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_ellipse(
                        self.center.to_lyon_point(),
                        self.radii.to_lyon_vector(),
                        Angle::zero(),
                        Winding::Positive,
                        &options,
                        output,
                    )
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
}

#[derive(Debug, PartialEq)]
pub struct PolygonShape {
    pub points: Vec<Vec2>,
    pub closed: bool,
}

impl Default for PolygonShape {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            closed: true,
        }
    }
}

impl ShapeSprite for PolygonShape {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        let points = self
            .points
            .iter()
            .map(|p| p.to_lyon_point())
            .collect::<Vec<Point>>();
        let polygon: Polygon<Point> = Polygon {
            points: points.as_slice(),
            closed: self.closed,
        };

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_polygon(polygon, &options, output)
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_polygon(polygon, &options, output)
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
}

#[derive(Debug)]
pub struct AABB {
    pos: glam::Vec2,
    size: glam::Vec2,
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            pos: glam::vec2(0.0, 0.0),
            size: glam::vec2(1.0, 1.0),
        }
    }
}

impl AABB {
    pub fn from_position_scale(position: glam::Vec2, scale: glam::Vec2) -> Self {
        Self::default().translate(position).scale(scale)
    }

    pub fn translate(&self, vector: glam::Vec2) -> Self {
        Self {
            pos: self.pos + vector,
            size: self.size,
        }
    }

    pub fn scale(&self, vector: glam::Vec2) -> Self {
        Self {
            pos: self.pos,
            size: self.size * vector,
        }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        ((self.pos.x - other.pos.x).abs() * 2.0 < (self.size.x + other.size.x))
            && ((self.pos.y - other.pos.y).abs() * 2.0 < (self.size.y + other.size.y))
    }
}

#[cfg(test)]
mod tests {
    use super::AABB;

    #[test]
    fn test_translation() {
        let aabb = AABB::default();
        let (x_translate, y_translate) = (2.0, 3.0);
        let translate_by = glam::vec2(x_translate, y_translate);
        let aabb2 = aabb.translate(translate_by);

        assert_eq!(aabb.size, aabb2.size);
        assert_eq!(aabb.pos.x + x_translate, aabb2.pos.x);
        assert_eq!(aabb.pos.y + y_translate, aabb2.pos.y);
    }

    #[test]
    fn test_scale() {
        let aabb = AABB::default();
        let (x_scale, y_scale) = (2.0, 3.0);
        let scale_by = glam::vec2(x_scale, y_scale);
        let aabb2 = aabb.scale(scale_by);

        assert_eq!(aabb.pos, aabb2.pos);
        assert_eq!(aabb.size.x * x_scale, aabb2.size.x);
        assert_eq!(aabb.size.y * y_scale, aabb2.size.y);

        let aabb3 = aabb2.scale(scale_by);
        assert_eq!(aabb2.pos, aabb3.pos);
        assert_eq!(aabb2.size.x * x_scale, aabb3.size.x);
        assert_eq!(aabb2.size.y * y_scale, aabb3.size.y);
    }

    #[test]
    fn test_intersect() {
        let aabb = AABB::default();
        let aabb2 = aabb.translate(glam::vec2(0.5, 0.5));
        let aabb3 = aabb.translate(glam::vec2(1.0, 1.0));
        let aabb4 = aabb.scale(glam::vec2(1.1, 1.1));

        assert!(aabb.intersect(&aabb));
        assert!(aabb.intersect(&aabb2));
        assert!(!aabb.intersect(&aabb3));
        assert!(aabb2.intersect(&aabb3));
        assert!(aabb4.intersect(&aabb));
        assert!(aabb4.intersect(&aabb2));
        assert!(aabb4.intersect(&aabb3));
    }
}

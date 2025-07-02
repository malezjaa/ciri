use three_d::{BoundingBox, ColorMaterial, Context, Gm};
use three_d_asset::{AxisAlignedBoundingBox, Srgba};

pub fn create_bounding_box(
    ctx: &Context,
    aabb: AxisAlignedBoundingBox,
    cm: ColorMaterial,
) -> Gm<BoundingBox, ColorMaterial> {
    Gm::new(BoundingBox::new(ctx, aabb), cm)
}

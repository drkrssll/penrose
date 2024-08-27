use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReflectHorizontal, ReserveTop},
        MainAndStack, Monocle,
    },
    core::layout::LayoutStack,
    stack,
};

const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.55;
const RATIO_STEP: f32 = 0.1;
const OUTER_PX: u32 = 2;
const INNER_PX: u32 = 2;
pub const BAR_PX: u32 = 20;

pub fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        ReflectHorizontal::wrap(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP)),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_PX))
}

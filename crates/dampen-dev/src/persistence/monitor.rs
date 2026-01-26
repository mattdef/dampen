/// Check if a window position is reasonable (likely on screen).
///
/// Since we can't easily detect monitors without a window handle or active event loop,
/// we use heuristics to filter out obviously invalid coordinates (e.g. from corrupted state
/// or disconnected monitors resulting in huge coordinates).
pub fn position_is_reasonable(x: i32, y: i32) -> bool {
    // Arbitrary bounds: -30,000 to +30,000
    // Most multi-monitor setups are within this range.
    const MAX_COORD: i32 = 30_000;
    const MIN_COORD: i32 = -30_000;

    (MIN_COORD..=MAX_COORD).contains(&x) && (MIN_COORD..=MAX_COORD).contains(&y)
}

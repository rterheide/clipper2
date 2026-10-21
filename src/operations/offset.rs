use clipper2c_sys::{clipper_clipperoffset, clipper_clipperoffset_add_paths64, clipper_clipperoffset_execute, clipper_clipperoffset_size, clipper_delete_paths64, clipper_paths64_size};
use crate::{malloc, EndType, JoinType, Paths, PointScaler};



/// This function offsets intpaths. TODO add more documentation.
///
/// All the paths are offset together, like `inflate` does, so a path that is a hole of another path, with the opposite orientation, is shrunk when the other path grows.
pub fn offset<P: PointScaler>(
    paths: impl Into<Paths<P>>,
    delta: f64,
    miter_limit: f64,
    join_type: JoinType,
    end_type: EndType,
    arc_tolerance: f64,
    preserve_collinear: i32,
    reverse_solution: i32,

) -> Paths<P>{
    let delta = P::scale(delta);
    let paths: Paths<P> = paths.into();

    unsafe{

        let paths_ptr = paths.to_clipperpaths64();

        let offseter_mem = malloc(clipper_clipperoffset_size());
        let offseter_ptr = clipper_clipperoffset(offseter_mem, miter_limit, arc_tolerance, preserve_collinear, reverse_solution);

        let result_mem = malloc(clipper_paths64_size());

        clipper_clipperoffset_add_paths64(offseter_ptr, paths_ptr, join_type.into(), end_type.into());
        let result_ptr = clipper_clipperoffset_execute(result_mem, offseter_ptr, delta);

        clipper_delete_paths64(paths_ptr);
        let result = Paths::from_clipperpaths64(result_ptr);
        return result;
    }

}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_offset() {
        let paths = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let expected_output = vec![vec![
            (1.41, -1.0),
            (-0.41, -1.0),
            (-1.0, -0.41),
            (-1.0, 1.41),
            (-0.41, 2.0),
            (1.41, 2.0),
            (2.0, 1.41),
            (2.0, -0.41)
        ]];

        let output: Vec<Vec<(f64, f64)>> =
        offset::<Centi>(paths, 1.0, 0.8, JoinType::Square, EndType::Polygon, 0.0, 1, 1).into();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_offset_shrinks_the_outline_and_grows_the_hole() {
        // The outline is counter clockwise and the hole clockwise.
        let paths = vec![
            vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)],
            vec![(3.0, 3.0), (3.0, 7.0), (7.0, 7.0), (7.0, 3.0)],
        ];

        let output = offset::<Centi>(paths, -0.5, 2.0, JoinType::Miter, EndType::Polygon, 0.0, 1, 0);

        let mut areas: Vec<f64> = output.iter().map(|path| path.signed_area()).collect();
        areas.sort_by(f64::total_cmp);
        // The outline of 10 x 10 gets 9 x 9, and the hole of 4 x 4 gets 5 x 5.
        assert_eq!(areas, vec![-25.0, 81.0]);
    }

    #[test]
    fn test_offset_reversed(){
        let paths = vec![(0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)];
        let expected_output = vec![vec![
            (0.55, 0.55),
            (0.45, 0.55),
            (0.45, 0.45),
            (0.55, 0.45)
        ]];

        let output: Vec<Vec<(f64, f64)>> =
        offset::<Centi>(paths, -0.45, 0.8, JoinType::Square, EndType::Polygon, 0.0, 1, 1).into();
        assert_eq!(output, expected_output);
    }
}
 
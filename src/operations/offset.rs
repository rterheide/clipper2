use clipper2c_sys::{clipper_clipperoffset, clipper_clipperoffset_add_path64, clipper_clipperoffset_execute, clipper_clipperoffset_size, clipper_delete_path64, clipper_delete_paths64, clipper_path64_size, clipper_paths64_inflate, clipper_paths64_size};
use crate::{malloc, EndType, JoinType, Paths, Path, PointScaler};



/// This function offsets intpaths. TODO add more documentation.
pub fn offset<P: PointScaler>(
    path: impl Into<Path<P>>,
    delta: f64,
    miter_limit: f64,
    join_type: JoinType,
    end_type: EndType,
    arc_tolerance: f64,
    preserve_collinear: i32,
    reverse_solution: i32,

) -> Paths<P>{
    let delta = P::scale(delta);
    let path: Path<P> = path.into();

    unsafe{

        let path_ptr = path.to_clipperpath64();

        let offseter_mem = malloc(clipper_clipperoffset_size());
        let offseter_ptr = clipper_clipperoffset(offseter_mem, miter_limit, arc_tolerance, preserve_collinear, reverse_solution);

        let result_mem = malloc(clipper_paths64_size());

        clipper_clipperoffset_add_path64(offseter_ptr, path_ptr, join_type.into(), end_type.into());
        let result_ptr = clipper_clipperoffset_execute(result_mem, offseter_ptr, delta);

        clipper_delete_path64(path_ptr);
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
 
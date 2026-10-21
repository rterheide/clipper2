use clipper2c_sys::{clipper_delete_paths64, clipper_paths64_inflate, clipper_paths64_size};

use crate::{EndType, JoinType, Paths, PointScaler, malloc};

/// This function performs both closed path and open path offsetting.
///
/// For closed paths passing a positive delta number will inflate the path
/// where passing a negative number will shrink the path.
///
/// **NOTE:** Inflate calls will frequently generate a large amount of very
/// close extra points and it is therefore recommented to almost always call
/// [`simplify`](./fn.simplify.html) on the path after inflating/shrinking it.
///
/// # Example
///
/// ```rust
/// use clipper2::*;
///
/// let paths: Paths = vec![(2.0, 2.0), (6.0, 2.0), (6.0, 10.0), (2.0, 6.0)].into();
///
/// let output = inflate(paths, 1.0, JoinType::Round, EndType::Polygon, 0.0);
/// let output = simplify(output, 0.01, false);
///
/// dbg!(output);
/// ```
///
/// ![Image displaying the result of the inflate example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/inflate.png)
///
/// For more details see the original [inflate paths](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/InflatePaths.htm) docs.
pub fn inflate<P: PointScaler>(
    paths: impl Into<Paths<P>>,
    delta: f64,
    join_type: JoinType,
    end_type: EndType,
    miter_limit: f64,
) -> Paths<P> {
    let delta = P::scale(delta);
    let paths: Paths<P> = paths.into();

    unsafe {
        let mem = malloc(clipper_paths64_size());
        let paths_ptr = paths.to_clipperpaths64();
        let result_ptr = clipper_paths64_inflate(
            mem,
            paths_ptr,
            delta,
            join_type.into(),
            end_type.into(),
            miter_limit,
        );
        clipper_delete_paths64(paths_ptr);
        let result = Paths::from_clipperpaths64(result_ptr);
        clipper_delete_paths64(result_ptr);
        result
    }
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_inflate() {
        let paths = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let expected_output = vec![vec![
            (2.0, -0.41),
            (2.0, 1.41),
            (1.41, 2.0),
            (-0.41, 2.0),
            (-1.0, 1.41),
            (-1.0, -0.41),
            (-0.41, -1.0),
            (1.41, -1.0),
        ]];

        let output: Vec<Vec<(f64, f64)>> =
            inflate::<Centi>(paths, 1.0, JoinType::Square, EndType::Polygon, 0.0).into();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_inflate_miter_limit_is_a_multiple_of_delta() {
        // A triangle with a sharp corner of about 30 degrees, which a miter join extends 1 / sin(15 degrees) = 3.9 times the delta.
        let triangle = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 2.7)];
        let area = |miter_limit: f64| -> f64 {
            inflate::<Centi>(triangle.clone(), 1.0, JoinType::Miter, EndType::Polygon, miter_limit)
                .signed_area()
        };

        // With a limit of 2 times the delta the sharp corner is squared off, with a limit of 10 it is not.
        assert!(area(2.0) < area(10.0) - 0.1);
    }
}

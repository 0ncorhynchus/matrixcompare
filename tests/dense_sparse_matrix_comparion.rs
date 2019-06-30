use matrixcompare::compare_matrices;
use matrixcompare::comparators::ExactElementwiseComparator;
use matrixcompare_mock::{dense_matrix_strategy_i64, sparse_matrix_strategy_i64};
use proptest::prelude::*;


proptest! {
    #[test]
    fn sparse_and_dense_matrices_should_compare_the_same_as_dense_dense(
        dense in dense_matrix_strategy_i64(0..5, 0..5),
        sparse in sparse_matrix_strategy_i64(0..5, 0..5)
    ) {
        let c = ExactElementwiseComparator;
        compare_matrices(&dense, &sparse, &c) == compare_matrices(&dense, sparse.to_dense().unwrap(), &c)
    }
}

use std::sync::Arc;

use crate::backprop::BackpropOp;
use crate::dim::Dimension;
use crate::storage::Storage;

pub struct Tensor {
    dimension: Dimension,
    storage: Arc<Storage>,
    backprop_op: BackpropOp,
}

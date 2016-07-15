pub enum RopResult<T, E> {
    Success(T, Option<Vec<E>>),
    Failure(Vec<E>),
}

pub fn succeed<T, E>(x: T) -> RopResult<T, E> {
    RopResult::Success(x, Option::None::<Vec<E>>)
}

pub fn succeed_with_msg<T, E>(x: T, msg: E) -> RopResult<T, E> {
    RopResult::Success(x, Option::Some(vec![msg]))
}

pub fn fail<T, E>(msg: E) -> RopResult<T, E> {
    RopResult::Failure(Option::Some(vec![msg]))
}
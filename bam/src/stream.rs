use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct CompiledStream<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iterator: I,
    state: Rc<RefCell<Vec<VecDeque<I::Item>>>>,
    stream_index: usize,
}

impl<I> Iterator for CompiledStream<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.state.borrow_mut();
        match state[self.stream_index].pop_back() {
            Some(x) => Some(x),
            None => match self.iterator.next() {
                Some(x) => {
                    for (i, buffer) in &mut state.iter_mut().enumerate() {
                        if i != self.stream_index {
                            buffer.push_front(x.clone())
                        };
                    }
                    Some(x)
                }
                None => None,
            },
        }
    }
}

impl<I> CompiledStream<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn duplicate(self) -> (Self, Self) {
        let state = Rc::new(RefCell::new(
            vec![VecDeque::new(), VecDeque::new()],
        ));
        let stream1 = CompiledStream {
            iterator: self.iterator,
            state: state.clone(),
            stream_index: 0,
        };
        let stream2 = CompiledStream {
            iterator: self.iterator,
            state,
            stream_index: 1,
        };
        (stream1, stream2)
    }

}

impl<A, B, I> CompiledStream<I>
where
    A: Clone,
    B: Clone,
    I: Iterator<Item = (A, B)>,
{
    fn unzip<L, R>(
        self
    ) -> (impl Iterator<Item=A>, impl Iterator<Item = B>)  {
        let (left, right) = Self::duplicate(self);
        (left.map(|(x, _)| x), right.map(|(_, y)| y))
    }
}
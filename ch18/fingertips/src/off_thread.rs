use std::{ops::ControlFlow, sync::mpsc, thread};

pub trait OffThreadExt: Iterator {
    /// 이 이터레이터를 스레드를 띄우는 이터레이터로 변환한다.
    /// `next()` 호출은 별도의 워커 스레드에서 일어나므로, 이터레이터의 루프와 본문이 동시에 실행된다.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);

        thread::spawn(move || {
            self.into_iter().try_for_each(|item| {
                if sender.send(item).is_err() {
                    return ControlFlow::Break(());
                }
                ControlFlow::Continue(())
            })
        });

        receiver.into_iter()
    }
}

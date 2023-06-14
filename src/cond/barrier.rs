
struct Barrier{
    mutex:Mutex,
    counters: (usize,usize),
    all_bros:bool,
    cv:CondVar;
}

impl Barrier{
    pub fn new(participants: usize) -> Self{
Barrier{mutex:Mutex::new(),counters: (participants,participants),all_bros:false,cv:CondVar::new()}
    }
    
    pub fn arrive(&self){
        let _guard = Guard::new(self.mutex);
        let bro = self.all_bros;
        self.counters.1 -= 1;

        if self.counters.1 == 0{
            self.all_bros = !self.all_bros;
            self.counters.1 = self.counters.0;
            cv.notify_all();
        }
        else{
            cv.wait(_guard, ||{return bro!=self.all_bros;})
        }
    }
}
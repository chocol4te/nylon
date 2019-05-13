use {
    libc::{
        mlockall, pthread_attr_init, pthread_attr_setstacksize, pthread_attr_t, pthread_create,
        pthread_getschedparam, pthread_self, pthread_setschedparam, pthread_t, sched_param,
        MCL_CURRENT, MCL_FUTURE, SCHED_FIFO,
    },
    std::mem,
};

const STACK_SIZE: usize = 8192;
const PRIORITY: i32 = 80;

fn main() {
    unsafe {
        let mut attr: pthread_attr_t = mem::uninitialized();
        let mut thread: pthread_t = mem::uninitialized();

        mlockall(MCL_CURRENT | MCL_FUTURE);
        pthread_attr_init(&mut attr);
        pthread_attr_setstacksize(&mut attr, STACK_SIZE);

        {
            let mut param: sched_param = mem::uninitialized();
            param.sched_priority = PRIORITY;
            pthread_setschedparam(thread, SCHED_FIFO, &param);
        }

        let mut null: core::ffi::c_void = mem::uninitialized();
        pthread_create(&mut thread, &attr, example_fn, &mut null);
    }

    println!(
        "Hello from main thread! pthread_self: {}",
        unsafe { pthread_self() },
        //0 //get_sched_params()
    );
}

extern "C" fn example_fn(ret: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    println!(
        "Hello from spawned thread! pthread_self: {}",
        unsafe { pthread_self() },
        //get_sched_params()
    );
    ret
}

/*
fn get_sched_params() -> () {
    unsafe {
        //let native = pthread_self();
        //let mut policy = 0i32;
        //let mut param: sched_param = mem::uninitialized();
        //pthread_getschedparam(native, &mut policy, &mut param);
        ()
    }
}
*/

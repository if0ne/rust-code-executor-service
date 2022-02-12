#[macro_use] extern crate rocket;

use std::mem;
use std::os::windows::io::AsRawHandle;
use rocket::http::Status;
use rocket::response::{status};
use winapi::um::psapi::PROCESS_MEMORY_COUNTERS;
use winapi::um::winnt::HANDLE;

#[get("/")]
async fn index<'o>() -> status::Custom<String> {
    //Надо как-то переделать под асинхронщину (tokio::process).
    //Сейчас проблемы при получении handle-а и вызова асинхронного wait()
    let mut process = std::process::Command::new("rustc").arg("test.txt").spawn().unwrap();
    let handle = process.as_raw_handle();
    //Чтобы измерить время
    let status = process.wait().unwrap();

    let (utime, stime) = unsafe {
        let mut ctime = mem::zeroed();
        let mut etime = mem::zeroed();
        let mut kernel_time = mem::zeroed();
        let mut user_time = mem::zeroed();
        let res = winapi::um::processthreadsapi::GetProcessTimes(
            handle,
            &mut ctime,
            &mut etime,
            &mut kernel_time,
            &mut user_time
        );

        if res != 0 {
            //Умножение на 100 для перевода в наносекунды, т.к. функция возвращает время измеряемой в 100 наносекундах
            let user = (((user_time.dwHighDateTime as i64) << 32)
                + user_time.dwLowDateTime as i64) * 100;
            let kernel = (((kernel_time.dwHighDateTime as i64) << 32)
                + kernel_time.dwLowDateTime as i64) * 100;
            //Перевод в секунды
            (user as u64 / 1000000000, kernel as u64 / 1000000000)
        } else {
            (0, 0)
        }
    };

    //В килобайтах
    let maxrss = unsafe {
        let mut pmc = mem::zeroed();
        let res = winapi::um::psapi::GetProcessMemoryInfo(
            handle as HANDLE,
            &mut pmc,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        );
        if res != 0 {
            pmc.PeakWorkingSetSize as u64
        } else {
            0
        }
    } / 1024;

    println!("Time: {}s\nKernel time: {}s\nMemory: {}KB", utime, stime, maxrss);

    status::Custom(Status::Ok, String::from("Test"))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await;
}
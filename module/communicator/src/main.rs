//라이브러리 크레이트를 가져오기 위한 명령어
extern crate communicator;


pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    communicator::client::connect();

    of::nested_modules();



}
use kvs::KvStore;

fn main() {
    let mut kvs = KvStore::open("./log/test.log").unwrap();

    kvs.set("name".to_owned(), "junhui".to_owned());
    kvs.set("age".to_owned(), "18".to_owned());
}
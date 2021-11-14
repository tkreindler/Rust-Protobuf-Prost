use std::error;

// required to use trait functions to encode the message
use prost::Message;

mod proto;

fn main() -> Result<(), Box<dyn error::Error>> {

    // create an example struct with the data
    let person = proto::Person {
        name: String::from("Tristan Kreindler"),
        id: 1,
        email: String::from("tristankreindler@gmail.com"),
        phones: vec![proto::person::PhoneNumber{
            number: String::from("xxx-xxx-xxxx"),
            r#type: proto::person::PhoneType::Mobile as i32,
        }],
    };

    // reserve a buffer of the correct size and encode it
    let buf: Vec<u8> = {
        let mut buf = Vec::new();

        buf.reserve(person.encoded_len());

        person.encode(&mut buf)?;

        buf
    };

    // person is now encoded in buf

    // deserialize it back into person2
    let person2 = proto::Person::decode(&buf as &[u8])?;
    
    // assert that these people are equal
    assert_eq!(person.name, person2.name);
    assert_eq!(person.id, person2.id);
    assert_eq!(person.email, person2.email);
    assert_eq!(person.phones.len(), person2.phones.len());

    for (i, phone) in person.phones.iter().enumerate() {
        let phone2 = &person2.phones[i];

        assert_eq!(phone.number, phone2.number);
        assert_eq!(phone.r#type, phone2.r#type)
    }

    Ok(())
}

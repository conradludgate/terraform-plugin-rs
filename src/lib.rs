pub mod pb {
    tonic::include_proto!("tfplugin5");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

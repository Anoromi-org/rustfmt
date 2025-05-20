fn new() -> Self
{
  let bytes = include_bytes!( "../../../assets/test-priv.der" );
  let key = RsaPrivateKey::from_pkcs8_der( bytes ).unwrap();
  let public_key = key.to_public_key();
  Self
  {
     public_key : public_key
  }
}

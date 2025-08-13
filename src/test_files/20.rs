impl Client for MockClient
{
  async fn exchange_token( &self, verifier : PkceCodeVerifier, nonce : Nonce, auth_data : QueryParams )
  ->
  ApplicationResult< () >
  {
  }
}

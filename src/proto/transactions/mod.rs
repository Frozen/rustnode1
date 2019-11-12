mod transfer;

//pub type TransactionType = u32;

pub enum TransactionType {
    GenesisTransaction,
    TransferTransaction,
}

//All transaction types supported.
//const (
//GenesisTransaction        TransactionType = iota + 1 // 1 - Genesis transaction
//PaymentTransaction                                   // 2 - Payment transaction
//IssueTransaction                                     // 3 - Issue transaction
const TRANSFER_TRANSACTION: u8 = 4; // 4 - Transfer transaction
                                    //ReissueTransaction                                   // 5 - Reissue transaction
                                    //BurnTransaction                                      // 6 - Burn transaction
                                    //ExchangeTransaction                                  // 7 - Exchange transaction
                                    //LeaseTransaction                                     // 8 - Lease transaction
                                    //LeaseCancelTransaction                               // 9 - LeaseCancel transaction
                                    //CreateAliasTransaction                               // 10 - CreateAlias transaction
                                    //MassTransferTransaction                              // 11 - MassTransfer transaction
                                    //DataTransaction                                      // 12 - Data transaction
                                    //SetScriptTransaction                                 // 13 - SetScript transaction
                                    //SponsorshipTransaction                               // 14 - Sponsorship transaction
                                    //SetAssetScriptTransaction                            // 15 - SetAssetScript transaction
                                    //InvokeScriptTransaction                              // 16 - InvokeScript transaction
                                    //)

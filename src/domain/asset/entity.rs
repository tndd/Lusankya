pub struct Asset {
    id: Uuid,
    version: Date,
    class: String,
    exchange: String,
    symbol: String,
    status: String,
    tradable: bool,
    marginable: bool,
    shortable: bool,
    easy_to_borrow: bool,
    fractionable: bool,
}
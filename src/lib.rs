use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;
use cw_orch::interface;

mod impl_into_contract {
    use super::*;

    #[cw_serde]
    pub struct QueryMsg(MockQueryMsg);

    impl From<MockQueryMsg> for QueryMsg {
        fn from(value: MockQueryMsg) -> Self {
            QueryMsg(value)
        }
    }

    #[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
    #[cfg_attr(feature = "interface", impl_into(QueryMsg))]
    #[derive(QueryResponses)]
    #[cw_serde]
    pub enum MockQueryMsg {
        #[returns(Empty)]
        Foo {},
        #[returns(u32)]
        Bar { x: u32 },
    }

    #[interface(Empty, Empty, QueryMsg, Empty)]
    pub struct MockContract;
}

mod contract {
    use super::*;

    #[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
    #[derive(QueryResponses)]
    #[cw_serde]
    pub enum QueryMsg {
        #[returns(Empty)]
        Foo {},
        #[returns(u32)]
        Bar { x: u32 },
    }

    #[interface(Empty, Empty, QueryMsg, Empty)]
    pub struct MockContract;
}

#[cfg(test)]
mod test {
    #![allow(unused)]

    use cw_orch::mock::Mock;

    fn impl_into_contract() -> Result<(), cw_orch::prelude::CwOrchError> {
        use crate::impl_into_contract::{MockContract, MockQueryMsgFns};

        let c = MockContract::new("contract_id", Mock::new("sender"));
        let foo = c.foo()?;
        let bar = c.bar(32)?;
        Ok(())
    }

    fn contract() -> Result<(), cw_orch::prelude::CwOrchError> {
        use crate::contract::{MockContract, QueryMsgFns};

        let c = MockContract::new("contract_id", Mock::new("sender"));
        let foo = c.foo()?;
        let bar = c.bar(32)?;
        Ok(())
    }
}

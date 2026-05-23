#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Address,
    Env,
    String,
    Symbol,
    Vec,
};

// ======================================================
// STRUCT
// ======================================================

// =====================
// PERSONAL TRANSACTION
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct PersonalTransaction {
    id: u64,

    owner: Address,

    tx_type: String,
    // income / expense

    category: String,

    title: String,

    amount: i128,

    description: String,

    timestamp: u64,
}

// =====================
// ASSET PORTFOLIO
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct AssetPortfolio {
    id: u64,

    owner: Address,

    asset_type: String,
    // stock / crypto

    symbol: String,
    // BTC, ETH, AAPL, BBCA

    quantity: i128,

    average_buy_price: i128,
}

// =====================
// MARKETPLACE TRANSACTION
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct MarketplaceTransaction {
    id: u64,

    owner: Address,

    platform: String,
    // Tokopedia / Shopee / Lazada

    transaction_type: String,
    // purchase / sale

    product_name: String,

    total: i128,

    timestamp: u64,
}

// =====================
// ORGANIZATION
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Organization {
    id: u64,

    name: String,

    owner: Address,
}

// =====================
// ORGANIZATION MEMBER
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct OrganizationMember {
    organization_id: u64,

    member: Address,

    role: String,
    // admin / treasurer / auditor / member
}

// =====================
// ORGANIZATION TRANSACTION
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct OrganizationTransaction {
    id: u64,

    organization_id: u64,

    created_by: Address,

    tx_type: String,
    // income / expense

    title: String,

    category: String,

    amount: i128,

    description: String,

    timestamp: u64,
}

// ======================================================
// STORAGE KEY
// ======================================================

const PERSONAL_TX: Symbol = symbol_short!("PERS_TX");

const ASSET_DATA: Symbol = symbol_short!("ASSET_DT");

const MARKET_DATA: Symbol = symbol_short!("MARKETDT");

const ORG_DATA: Symbol = symbol_short!("ORG_DATA");

const ORG_MEMBER: Symbol = symbol_short!("ORG_MEMB");

const ORG_TX: Symbol = symbol_short!("ORG_TX");

// ======================================================
// CONTRACT
// ======================================================

#[contract]
pub struct FinanceTransparencyContract;

// ======================================================
// IMPLEMENTATION
// ======================================================

#[contractimpl]
impl FinanceTransparencyContract {

    // ==================================================
    // PERSONAL FINANCE
    // ==================================================

    pub fn create_personal_transaction(
        env: Env,

        owner: Address,

        tx_type: String,

        category: String,

        title: String,

        amount: i128,

        description: String,
    ) -> String {

        owner.require_auth();

        let mut transactions: Vec<PersonalTransaction> =
            env.storage()
                .instance()
                .get(&PERSONAL_TX)
                .unwrap_or(Vec::new(&env));

        let transaction = PersonalTransaction {
            id: env.prng().gen::<u64>(),

            owner,

            tx_type,

            category,

            title,

            amount,

            description,

            timestamp: env.ledger().timestamp(),
        };

        transactions.push_back(transaction);

        env.storage()
            .instance()
            .set(&PERSONAL_TX, &transactions);

        String::from_str(
            &env,
            "Personal transaction berhasil ditambahkan"
        )
    }

    pub fn get_personal_transactions(
        env: Env
    ) -> Vec<PersonalTransaction> {

        env.storage()
            .instance()
            .get(&PERSONAL_TX)
            .unwrap_or(Vec::new(&env))
    }

    // ==================================================
    // ASSET PORTFOLIO
    // ==================================================

    pub fn add_asset(
        env: Env,

        owner: Address,

        asset_type: String,

        symbol: String,

        quantity: i128,

        average_buy_price: i128,
    ) -> String {

        owner.require_auth();

        let mut assets: Vec<AssetPortfolio> =
            env.storage()
                .instance()
                .get(&ASSET_DATA)
                .unwrap_or(Vec::new(&env));

        let asset = AssetPortfolio {
            id: env.prng().gen::<u64>(),

            owner,

            asset_type,

            symbol,

            quantity,

            average_buy_price,
        };

        assets.push_back(asset);

        env.storage()
            .instance()
            .set(&ASSET_DATA, &assets);

        String::from_str(
            &env,
            "Asset berhasil ditambahkan"
        )
    }

    pub fn get_assets(
        env: Env
    ) -> Vec<AssetPortfolio> {

        env.storage()
            .instance()
            .get(&ASSET_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ==================================================
    // MARKETPLACE TRANSACTION
    // ==================================================

    pub fn add_marketplace_transaction(
        env: Env,

        owner: Address,

        platform: String,

        transaction_type: String,

        product_name: String,

        total: i128,
    ) -> String {

        owner.require_auth();

        let mut transactions: Vec<MarketplaceTransaction> =
            env.storage()
                .instance()
                .get(&MARKET_DATA)
                .unwrap_or(Vec::new(&env));

        let transaction = MarketplaceTransaction {
            id: env.prng().gen::<u64>(),

            owner,

            platform,

            transaction_type,

            product_name,

            total,

            timestamp: env.ledger().timestamp(),
        };

        transactions.push_back(transaction);

        env.storage()
            .instance()
            .set(&MARKET_DATA, &transactions);

        String::from_str(
            &env,
            "Marketplace transaction berhasil ditambahkan"
        )
    }

    pub fn get_marketplace_transactions(
        env: Env
    ) -> Vec<MarketplaceTransaction> {

        env.storage()
            .instance()
            .get(&MARKET_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ==================================================
    // ORGANIZATION
    // ==================================================

    pub fn create_organization(
        env: Env,

        owner: Address,

        name: String,
    ) -> String {

        owner.require_auth();

        let mut organizations: Vec<Organization> =
            env.storage()
                .instance()
                .get(&ORG_DATA)
                .unwrap_or(Vec::new(&env));

        let organization = Organization {
            id: env.prng().gen::<u64>(),

            name,

            owner,
        };

        organizations.push_back(organization);

        env.storage()
            .instance()
            .set(&ORG_DATA, &organizations);

        String::from_str(
            &env,
            "Organization berhasil dibuat"
        )
    }

    pub fn get_organizations(
        env: Env
    ) -> Vec<Organization> {

        env.storage()
            .instance()
            .get(&ORG_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ==================================================
    // ORGANIZATION MEMBER
    // ==================================================

    pub fn add_organization_member(
        env: Env,

        organization_id: u64,

        member: Address,

        role: String,
    ) -> String {

        member.require_auth();

        let mut members: Vec<OrganizationMember> =
            env.storage()
                .instance()
                .get(&ORG_MEMBER)
                .unwrap_or(Vec::new(&env));

        let new_member = OrganizationMember {
            organization_id,

            member,

            role,
        };

        members.push_back(new_member);

        env.storage()
            .instance()
            .set(&ORG_MEMBER, &members);

        String::from_str(
            &env,
            "Member organization berhasil ditambahkan"
        )
    }

    pub fn get_organization_members(
        env: Env
    ) -> Vec<OrganizationMember> {

        env.storage()
            .instance()
            .get(&ORG_MEMBER)
            .unwrap_or(Vec::new(&env))
    }

    // ==================================================
    // ORGANIZATION TRANSACTION
    // ==================================================

    pub fn create_organization_transaction(
        env: Env,

        organization_id: u64,

        created_by: Address,

        tx_type: String,

        title: String,

        category: String,

        amount: i128,

        description: String,
    ) -> String {

        created_by.require_auth();

        let mut transactions: Vec<OrganizationTransaction> =
            env.storage()
                .instance()
                .get(&ORG_TX)
                .unwrap_or(Vec::new(&env));

        let transaction = OrganizationTransaction {
            id: env.prng().gen::<u64>(),

            organization_id,

            created_by,

            tx_type,

            title,

            category,

            amount,

            description,

            timestamp: env.ledger().timestamp(),
        };

        transactions.push_back(transaction);

        env.storage()
            .instance()
            .set(&ORG_TX, &transactions);

        String::from_str(
            &env,
            "Organization transaction berhasil ditambahkan"
        )
    }

    pub fn get_organization_transactions(
        env: Env
    ) -> Vec<OrganizationTransaction> {

        env.storage()
            .instance()
            .get(&ORG_TX)
            .unwrap_or(Vec::new(&env))
    }

    // ==================================================
    // DELETE PERSONAL TRANSACTION
    // ==================================================

    pub fn delete_personal_transaction(
        env: Env,

        id: u64
    ) -> String {

        let mut transactions: Vec<PersonalTransaction> =
            env.storage()
                .instance()
                .get(&PERSONAL_TX)
                .unwrap_or(Vec::new(&env));

        for i in 0..transactions.len() {

            if transactions.get(i).unwrap().id == id {

                transactions.remove(i);

                env.storage()
                    .instance()
                    .set(&PERSONAL_TX, &transactions);

                return String::from_str(
                    &env,
                    "Transaction berhasil dihapus"
                );
            }
        }

        String::from_str(
            &env,
            "Transaction tidak ditemukan"
        )
    }
}

mod test;
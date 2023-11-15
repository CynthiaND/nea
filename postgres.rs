use postgres::{Client, NoTls};
use std::Error;

fn client() {
    Client::connect(
        "host=localhost user=snow pass=rustontop dbname=flake",
        NoTls,
    )?;
}

pub fn init_tables() -> Result<(), Error> {
    let client = client();
    let mut transaction = client.transaction()?;
    transaction.batch_execute(
        "
    	CREATE TABLE users (
    		user_id UUID PRIMARY KEY NOT NULL,
    		username CHAR(20) NOT NULL,
    		password CHAR(16) NOT NULL,
    		salt CHAR(16) NOT NULL,
    		pin CHAR(10) NOT NULL,
    		address CHAR(106) NOT NULL,
            order_ids UUID[],
            );
        CREATE TABLE user_settings (
            user_id UUID PRIMARY KEY REFERENCES users,
            lang CHAR(2) DEFAULT 'EN',
            fiat CHAR(3) DEFAULT 'USD',
            dark_mode BOOLEAN DEFAULT TRUE NOT NULL,
            two_factor_authentication DEFAULT FALSE,
            pgp_key VARCHAR(4096),
            );
        CREATE TABLE vendors (
            vendor_id UUID PRIMARY NOT NULL,
            username CHAR(20) NOT NULL,
            password CHAR(16) NOT NULL,
            salt CHAR(16) NOT NULL,
            pin CHAR(10) NOT NULL,
            rating SMALLINT NOT NULL,
            address CHAR(106) NOT NULL,
            listing_ids UUID[] NOT NULL,
            order_ids UUID[],
            dispute_ids UUID[],
            );
        CREATE TABLE vendor_settings (
            vendor_id UUID PRIMARY KEY REFERENCES vendors,
            lang CHAR(2) DEFAULT 'EN',
            fiat CHAR(3) DEFAULT 'USD',
            dark_mode BOOLEAN DEFAULT TRUE,
            pgp_key CHAR(4096) NOT NULL,
            away BOOLEAN DEFAULT FALSE,
            );
        CREATE TABLE orders (
            order_id UUID PRIMARY KEY NOT NULL,
            items TEXT[] NOT NULL,
            payment_state CHAR(1) NOT NULL,
            vendor_id UUID REFERENCES vendors NOT NULL,
            user_id UUID REFERENCES users NOT NULL,
            af DATE NOT NULL,
            );
        CREATE TABLE listings (
            listing_id UUID PRIMARY KEY NOT NULL,
            name CHAR(50) NOT NULL,
            listing_type CHAR(10) NOT NULL,
            price SMALLINT NOT NULL,
            amounts SMALLINT[],
            limit SMALLINT,
            vendor_id UUID REFERENCES vendors,
            );
        CREATE TABLE disputes (
            dispute_id UUID PRIMARY KEY,
            order_id UUID REFERENCES orders,
            vendor_id REFERENCES vendors,
            state CHAR(1) NOT NULL,
            payment_state CHAR(1) NOT NULL,
            af DATE NOT NULL,
            );
        CREATE TABLE forums (
            forum_id UUID PRIMARY KEY NOT NULL,
            title CHAR(20) NOT NULL,
            description TEXT
            );
        CREATE TABLE posts (
            post_id UUID PRIMARY KEY NOT NULL,
            title CHAR(50) NOT NULL,
            body TEXT NOT NULL,
            rating SMALLINT DEFAULT 0,
            media TEXT[],
            forum_id UUID REFERENCES forums,
            private BOOLEAN DEFAULT FALSE,
            );
        ",
    );
    transaction.commit()?;
}

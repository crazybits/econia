/// Pure-Move implementation of user-side open orders functionality
module Econia::Orders {

    // Uses >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    use Econia::CritBit::{
        CB,
        empty as cb_e
    };

    use Std::Signer::{
        address_of as s_a_o
    };

    // Uses <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    // Test-only uses >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[test_only]
    use Econia::CritBit::{
        is_empty as cb_i_e
    };

    // Test-only uses <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    // Structs >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    /// Friend-like capability, administered instead of declaring as a
    /// friend a module containing Aptos native functions, which would
    /// inhibit coverage testing via the Move CLI. See `Econia::Caps`
    struct FriendCap has copy, drop, store {}

    /// Open orders, for the given market, on a user's account
    struct OO<phantom B, phantom Q, phantom E> has key {
        /// Scale factor
        f: u64,
        /// Asks
        a: CB<u64>,
        /// Bids
        b: CB<u64>
    }

    // Structs <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    // Test-only structs >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[test_only]
    /// Base coin type
    struct BT{}

    #[test_only]
    /// Quote coin type
    struct QT{}

    #[test_only]
    /// Scale exponent type
    struct ET{}

    // Test-only structs <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    // Error codes >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    /// When open orders already exists at given address
    const E_ORDERS_EXISTS: u64 = 0;
    /// When order book does not exist at given address
    const E_NO_ORDERS: u64 = 1;
    /// When account/address is not Econia
    const E_NOT_ECONIA: u64 = 2;

    // Error codes <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    // Public functions >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    /// Return `true` if specified open orders type exists at address
    public fun exists_orders<B, Q, E>(
        a: address
    ): bool {
        exists<OO<B, Q, E>>(a)
    }

    /// Return a `FriendCap`, aborting if not called by Econia
    public fun get_friend_cap(
        account: &signer
    ): FriendCap {
        // Assert called by Econia
        assert!(s_a_o(account) == @Econia, E_NOT_ECONIA);
        FriendCap{} // Return requested capability
    }

    /// Initialize open orders under host account, provided `FriendCap`,
    /// with market types `B`, `Q`, `E`, and scale factor `f`
    public fun init_orders<B, Q, E>(
        user: &signer,
        f: u64,
        _c: FriendCap
    ) {
        // Assert open orders does not already exist under user account
        assert!(!exists_orders<B, Q, E>(s_a_o(user)), E_ORDERS_EXISTS);
        // Pack empty open orders container
        let o_o = OO<B, Q, E>{f, a: cb_e<u64>(), b: cb_e<u64>()};
        move_to<OO<B, Q, E>>(user, o_o); // Move to user
    }

    /// Return scale factor of specified open orders at given address
    public fun scale_factor<B, Q, E>(
        addr: address
    ): u64
    acquires OO {
        // Assert open orders container exists at given address
        assert!(exists_orders<B, Q, E>(addr), E_NO_ORDERS);
        // Return open order container's scale factor
        borrow_global<OO<B, Q, E>>(addr).f
    }

    // Public functions <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

    // Tests >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[test(account = @TestUser)]
    #[expected_failure(abort_code = 2)]
    /// Verify failure for non-Econia account
    fun get_friend_cap_failure(
        account: &signer
    ) {
        get_friend_cap(account); // Attempt invalid invocation
    }

    #[test(econia = @Econia)]
    /// Verify success for Econia account
    fun get_friend_cap_success(
        econia: &signer
    ) {
        // Unpack result of valid invocation
        let FriendCap{} = get_friend_cap(econia);
    }

    #[test(user = @TestUser)]
    #[expected_failure(abort_code = 0)]
    /// Verify failed re-initialization of open orders container
    fun init_orders_failure_exists(
        user: &signer,
    ) {
        // Initialize open orders with scale factor 1
        init_orders<BT, QT, ET>(user, 1, FriendCap{});
        // Attempt invalid re-initialization
        init_orders<BT, QT, ET>(user, 1, FriendCap{});
    }

    #[test(user = @TestUser)]
    /// Verify successful initialization of open orders container
    fun init_orders_success(
        user: &signer,
    ) acquires OO {
        // Initialize open orders with scale factor 1
        init_orders<BT, QT, ET>(user, 1, FriendCap{});
        let user_addr = s_a_o(user); // Get user address
        // Assert open orders exists and has correct scale factor
        assert!(scale_factor<BT, QT, ET>(user_addr) == 1, 0);
        // Borrow immutable reference to open orders
        let o_o = borrow_global<OO<BT, QT, ET>>(user_addr);
        // Assert bid and ask trees init empty
        assert!(cb_i_e(&o_o.a) && cb_i_e(&o_o.b), 2);
    }

    #[test]
    #[expected_failure(abort_code = 1)]
    /// Verify failure for no orders
    fun scale_factor_failure()
    acquires OO {
        scale_factor<BT, QT, ET>(@TestUser); // Attempt invalid query
    }

    // Tests <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
}
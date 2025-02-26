fn f() -> i32 {
    var v: i32;
    let a = 5;
    const ca = 5;
    {
        var vv: u32;
        let ba = a + a;
        const cba = ca + ca;
        {
            var vvv: u32;
        }
        return ba;
    }
}
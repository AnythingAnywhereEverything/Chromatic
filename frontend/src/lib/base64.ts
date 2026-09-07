function b64ToBn(b64: string) {
    var bin = atob(b64);
    var hex: string[] = [];

    bin.split("").forEach(function (ch) {
        var h = ch.charCodeAt(0).toString(16);
        if (h.length % 2) {
            h = "0" + h;
        }
        hex.push(h);
    });

    return BigInt("0x" + hex.join(""));
}

function bnToB64(bn: bigint) {
    var hex = BigInt(bn).toString(16);
    if (hex.length % 2) {
        hex = "0" + hex;
    }

    var bin = [];
    var i = 0;
    var d;
    var b;
    while (i < hex.length) {
        d = parseInt(hex.slice(i, i + 2), 16);
        b = String.fromCharCode(d);
        bin.push(b);
        i += 2;
    }

    return btoa(bin.join(""));
}

function base64ToUrlBase64(str: string) {
    return str.replace(/\+/g, "-").replace(/\//g, "_").replace(/=/g, "");
}

function urlBase64ToBase64(str: string) {
    var r = str.length % 4;
    if (2 === r) {
        str += "==";
    } else if (3 === r) {
        str += "=";
    }
    return str.replace(/-/g, "+").replace(/_/g, "/");
}

export { b64ToBn, bnToB64, base64ToUrlBase64, urlBase64ToBase64 };

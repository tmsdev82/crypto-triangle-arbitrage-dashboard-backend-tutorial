# Rust Warp websocket crypto triangle arbitrage dashboard backend

This repository is the end result of a series of articles on my blog describing how to create a backend that calculates crypto trading triangle arbitrage profits, and how to send that data to a connected client. This example code shows how to gather data from the Binance websocket API for BTC->ETH->BNB->BTC trades, and how to calculate potential profits. The articles can be found on my blog here: 
* [TMS Blog - Crypto triangle arbitrage dashboard: how to, part 1](https://tms-dev-blog.com/crypto-triangle-arbitrage-dashboard-how-to-part-1/)
* [Crypto triangle arbitrage: how to part 2: frontend](https://tms-dev-blog.com/crypto-triangle-arbitrage-how-to-part-2-frontend/) (only frontend work)
* [TMS Blog - Crypto triangle arbitrage dashboard, part 3: how to add logging](https://tms-dev-blog.com/crypto-triangle-arbitrage-logging-for-backend/)

WebSocket clients will be able to connect to 127.0.0.1:8000/ws, when connected the client will receive JSON data like the following:

```json
{
  "mid_pair_data" : {
    "stream" : "bnbeth@depth5@100ms",
    "data" : {
      "bids" : [
        {
          "size" : 0.23200000000000001,
          "price" : 0.12917000000000001
        },
        {
          "size" : 0.27400000000000002,
          "price" : 0.12914
        },
        {
          "size" : 3.6579999999999999,
          "price" : 0.12912000000000001
        },
        {
          "size" : 32.420000000000002,
          "price" : 0.12911
        },
        {
          "size" : 11.035,
          "price" : 0.12909999999999999
        }
      ],
      "lastUpdateId" : 933056340,
      "asks" : [
        {
          "size" : 0.13900000000000001,
          "price" : 0.12917999999999999
        },
        {
          "size" : 4.7930000000000001,
          "price" : 0.12920999999999999
        },
        {
          "size" : 37.323,
          "price" : 0.12922
        },
        {
          "size" : 0.111,
          "price" : 0.12923000000000001
        },
        {
          "size" : 0.126,
          "price" : 0.12926000000000001
        }
      ]
    }
  },
  "start_pair_data" : {
    "stream" : "ethbtc@depth5@100ms",
    "data" : {
      "bids" : [
        {
          "size" : 21.669,
          "price" : 0.065795000000000006
        },
        {
          "size" : 0.28599999999999998,
          "price" : 0.065790000000000001
        },
        {
          "size" : 0.64600000000000002,
          "price" : 0.065789
        },
        {
          "size" : 0.085000000000000006,
          "price" : 0.065786999999999998
        },
        {
          "size" : 6.1219999999999999,
          "price" : 0.065783999999999995
        }
      ],
      "lastUpdateId" : 3807060057,
      "asks" : [
        {
          "size" : 4.907,
          "price" : 0.065795999999999993
        },
        {
          "size" : 3.77,
          "price" : 0.065797999999999995
        },
        {
          "size" : 5.3659999999999997,
          "price" : 0.065803
        },
        {
          "size" : 1.077,
          "price" : 0.065804000000000001
        },
        {
          "size" : 0.153,
          "price" : 0.065805000000000002
        }
      ]
    }
  },
  "end_pair_data" : {
    "stream" : "bnbbtc@depth5@100ms",
    "data" : {
      "bids" : [
        {
          "size" : 125.37,
          "price" : 0.0085000000000000006
        },
        {
          "size" : 17.870000000000001,
          "price" : 0.0084989999999999996
        },
        {
          "size" : 3.1099999999999999,
          "price" : 0.0084969999999999993
        },
        {
          "size" : 39.479999999999997,
          "price" : 0.0084960000000000001
        },
        {
          "size" : 74.170000000000002,
          "price" : 0.0084950000000000008
        }
      ],
      "lastUpdateId" : 2109002030,
      "asks" : [
        {
          "size" : 6.04,
          "price" : 0.0085019999999999991
        },
        {
          "size" : 18.34,
          "price" : 0.0085030000000000001
        },
        {
          "size" : 4.6600000000000001,
          "price" : 0.0085039999999999994
        },
        {
          "size" : 10.869999999999999,
          "price" : 0.0085050000000000004
        },
        {
          "size" : 81.340000000000003,
          "price" : 0.0085059999999999997
        }
      ]
    }
  },
  "triangle" : [
    "btc",
    "eth",
    "bnb"
  ],
  "profits" : [
    0.000055617210749314694,
    -0.0003245892775527448,
    -0.0007131147363421686,
    -0.0009232193830603919,
    -0.0012878390376491744
  ]
}
```

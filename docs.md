### Liquidity strategies

```ninescript
lqstrategy(title = "Sample liquidity strategy"...)

strategy.provide_liqudity(id, min=x, max=y, qty=z, fee=w, ...)
strategy.close_liquidity(id)
strategy.close_all()
```

Requirements:

- **Liquidity position can only be run against symbol type `liq_pool`**
- **OHLC represents the underlying price, V represents traded voulme that is directly used for profit making**
- **Liquidity position must be closed and reopened when changing the range (this behavior may potenially be changed in future)**
- **Despite of an opportunity to open leveraged positions, it is usually not possible to do the same in real world smart contracts.**

### Performance manipulation

You can manipulate different values to get **fair results**

```ninesript
import ninescript/1/value_manipulation

strategy.equity += 100
strategy.equity -= 100
strategy.opentrades.size(0) -= 100
```

### Complex strategy evaluations

Ninescript can take additional data series like `liquidity pool ratio` and `premium rate`,

**These values when provided are used for strategy evalutaions by default.**

You can disable this behavior by code
```ninescript
strategy(enable_liquidity_ratio = false, enable_funding = false)
```

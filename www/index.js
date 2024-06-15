import * as symbols from "symbols";

function parseFutureContract(future_contract) {
  if (!future_contract) {
    return null;
  }

  return {
    continuous: future_contract.continuous,
    month: future_contract.month,
    year: future_contract.year,
  };
}

function parseOptionContract(option_contract) {
  if (!option_contract) {
    return null;
  }
  return {
    strike_price: option_contract.strike_price,
    put_call: option_contract.put_call,
    date: option_contract.date,
  };
}

const { createApp, ref } = Vue;

const SYMBOL_TYPE_MAPPING = {
    0: 'Stocks',
    1: 'Futures',
    2: 'Future Options',
    3: 'Stock Options',
    4: 'Unknown Type'
}

const PUT_CALL_MAPPING = {
    0: 'PUT',
    1: 'CALL'
}

createApp({
  setup() {
    const symbolInfo = ref(null);
    const symbol = ref("");
    const error = ref(null);
    const onSubmit = (e) => {
      symbolInfo.value = null;
      error.value = null;
      // if (symbolInfo.value) {
      //     symbolInfo.free()
      // }
      try {
        performance.mark("startParse");
        let info = symbols.parse_symbol_js(symbol.value);
        symbolInfo.value = {
          symbol_type: info.symbol_type,
          original_symbol: info.original_symbol,
          underlying_symbol: info.underlying_symbol,
          symbol_modifier: info.symbol_modifier,
          option_contract: parseOptionContract(info.option_contract),
          future_contract: parseFutureContract(info.future_contract),
        };
        performance.mark("endParse");

        console.info(
          `Parsed in: ${
            performance.measure("parsing", "startParse", "endParse").duration
          }ms`
        );
      } catch (e) {
        error.value = e.toString();
      }
    };
    return {
        onSubmit,
        symbolInfo,
        symbol,
        SYMBOL_TYPE_MAPPING,
        PUT_CALL_MAPPING,
        error,
    }
  }
}).mount('#app')

import * as symbols from "symbols";
window.parse_symbol_js = symbols.parse_symbol_js;
function parseFutureContract(future_contract) {
  if (!future_contract) {
    return null;
  }

  return {
    continuous: future_contract.continuous,
    month: future_contract.month,
    year: future_contract.year,
    expiration: future_contract.expiration
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
        symbolInfo.value = symbols.parse_symbol_js(symbol.value);
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

    const useExample = (e) => {
      symbol.value = e.target.innerText;
      onSubmit()
    }
    return {
        onSubmit,
        useExample,
        symbolInfo,
        symbol,
        SYMBOL_TYPE_MAPPING,
        PUT_CALL_MAPPING,
        error,
    }
  }
}).mount('#app')

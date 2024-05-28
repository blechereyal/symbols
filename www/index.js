import * as symbols from "symbols";

function parseFutureContract(future_contract) {
    if (!future_contract) {
        return null;
    }

    return {
        continuous: future_contract.continuous,
        month: future_contract.month,
        year: future_contract.year,
    }
}

function parseOptionContract(option_contract) {
    if (!option_contract) {
        return null;
    }
    return {
        strike_price: option_contract.strike_price,
        put_call: option_contract.put_call, 
        date: option_contract.date,
    }
}


const { createApp, ref } = Vue

createApp({
  setup() {
    const symbolInfo = ref(null);
    const symbol = ref("");
    const onSubmit = (e) => {
        // if (symbolInfo.value) {
        //     symbolInfo.free()
        // }
        performance.mark('startParse');
        let info = symbols.parse_symbol_js(symbol.value);
        symbolInfo.value = {
            symbol_type: info.symbol_type, 
            original_symbol: info.original_symbol, 
            underlying_symbol: info.underlying_symbol, 
            symbol_modifier: info.symbol_modifier, 
            option_contract: parseOptionContract(info.option_contract), 
            future_contract: parseFutureContract(info.future_contract), 
        } 
        performance.mark('endParse');
        
        console.info(`Parsed in: ${performance.measure('parsing', 'startParse', 'endParse').duration}ms`);
    }
    return {
        onSubmit,
        symbolInfo,
        symbol
    }
  }
}).mount('#app')
(function() {
    if (typeof WebAssembly === "object" && typeof WebAssembly.instantiate === "function") {
      return;
    } else {
      window.location.href = "https://old.based.radio";
    }
  })();
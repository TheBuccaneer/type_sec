pub enum FailKind { F1_DoubleWait, F2_MissWait, F3_ReadInFlight, F4_Order, F5_CrossContext, F6_Leak, F7_InvalidSize, F8_Other }

pub fn inject(_k: FailKind) {
    // Platzhalter: Jede Variante bekommt ein minimales Host-Snippet,
    // das das jeweilige Fehlverhalten demonstriert (Baseline).
}

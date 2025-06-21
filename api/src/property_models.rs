use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "property_type", rename_all = "lowercase")]
pub enum PropertyType {
    SingleFamily,
    MultiFamily,
    Apartment,
    Condominium,
    Townhouse,
    Duplex,
    Triplex,
    Fourplex,
    Commercial,
    MixedUse,
    VacationRental,
    MobileHome,
    StudentHousing,
    SeniorHousing,
    CoLivingSpace,
    StorageUnit,
    ParkingSpace,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "furnishing_type", rename_all = "lowercase")]
pub enum FurnishingType {
    Unfurnished,
    PartiallyFurnished,
    FullyFurnished,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "rental_status", rename_all = "lowercase")]
pub enum RentalStatus {
    Vacant,
    Occupied,
    UnderMaintenance,
    UnderRenovation,
    Listed,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "lease_term", rename_all = "lowercase")]
pub enum LeaseTerm {
    Monthly,
    Quarterly,
    Biannual,
    Annual,
    FixedTerm,
    MonthToMonth,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "payment_method", rename_all = "lowercase")]
pub enum PaymentMethod {
    Cash,
    Check,
    BankTransfer,
    CreditCard,
    DebitCard,
    OnlinePayment,
    MoneyOrder,
}


// ROI-Specific Enums
#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "expense_category", rename_all = "lowercase")]
pub enum ExpenseCategory {
    Mortgage,
    PropertyTax,
    Insurance,
    HoaFees,
    Utilities,
    Maintenance,
    Repairs,
    PropertyManagement,
    Advertising,
    LegalFees,
    Accounting,
    CapitalImprovements,
    VacancyReserve,
    Other,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "income_category", rename_all = "lowercase")]
pub enum IncomeCategory {
    Rent,
    LateFees,
    ParkingFees,
    StorageFees,
    Laundry,
    PetFees,
    ApplicationFees,
    LeaseBreakFees,
    Other,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "tax_deduction_category", rename_all = "lowercase")]
pub enum TaxDeductionCategory {
    Depreciation,
    Interest,
    Repairs,
    HomeOffice,
    Travel,
    Education,
    ProfessionalServices,
    Other,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "roi_metric", rename_all = "lowercase")]
pub enum RoiMetric {
    CashOnCash,
    CapRate,
    GrossRentMultiplier,
    NetOperatingIncome,
    InternalRateOfReturn,
    EquityBuildRate,
    AppreciationRate,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "vacancy_calculation_method", rename_all = "lowercase")]
pub enum VacancyCalculationMethod {
    FixedRate,          // e.g., 5% of gross rents
    HistoricalAverage,  // based on property history
    MarketAverage,      // based on local market
    CustomDays,         // specific number of days vacant
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "appreciation_method", rename_all = "lowercase")]
pub enum AppreciationMethod {
    None,
    FixedAnnualRate,
    HistoricalMarketAverage,
    CpiAdjusted,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "financing_type", rename_all = "lowercase")]
pub enum FinancingType {
    Cash,
    ConventionalLoan,
    FHA,
    VA,
    HardMoney,
    PrivateMoney,
    SellerFinancing,
    HELOC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyFinancials {
    pub purchase_price: f64,
    pub after_repair_value: Option<f64>,
    pub purchase_closing_costs: f64,
    pub rehab_costs: f64,
    pub financing_type: FinancingType,
    pub loan_details: Option<LoanDetails>,
    pub annual_appreciation_rate: f64,
    pub appreciation_method: AppreciationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanDetails {
    pub down_payment_percent: f64,
    pub interest_rate: f64,
    pub loan_term_years: i32,
    pub points: f64,
    pub other_loan_fees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingExpenses {
    pub property_taxes_annual: f64,
    pub insurance_annual: f64,
    pub maintenance_rate: f64, // % of monthly rent
    pub vacancy_rate: f64,
    pub vacancy_method: VacancyCalculationMethod,
    pub property_management_rate: f64, // % of monthly rent
    pub hoa_fees_monthly: Option<f64>,
    pub utilities_responsibility: UtilitiesResponsibility,
    pub other_expenses: Vec<CustomExpense>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomExpense {
    pub category: ExpenseCategory,
    pub amount: f64,
    pub frequency: ExpenseFrequency,
    pub tax_deductible: bool,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "expense_frequency", rename_all = "lowercase")]
pub enum ExpenseFrequency {
    Monthly,
    Quarterly,
    Annual,
    OneTime,
    PerTenant,
    PerUnit,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "utilities_responsibility", rename_all = "lowercase")]
pub enum UtilitiesResponsibility {
    OwnerPaysAll,
    TenantPaysAll,
    Split, // Specify which utilities are paid by whom
}
/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#pragma once

namespace qaul {

	using namespace System;
	using namespace System::ComponentModel;
	using namespace System::Collections;
	using namespace System::Windows::Forms;
	using namespace System::Data;
	using namespace System::Drawing;

	/// <summary>
	/// Zusammenfassung für formError
	/// </summary>
	public ref class formError : public System::Windows::Forms::Form
	{
	public:
		formError(void)
		{
			InitializeComponent();
			//
			//TODO: Konstruktorcode hier hinzufügen.
			//
		}

	protected:
		/// <summary>
		/// Verwendete Ressourcen bereinigen.
		/// </summary>
		~formError()
		{
			if (components)
			{
				delete components;
			}
		}
	private: System::Windows::Forms::Label^  lblError;
	protected: 

	private: System::Windows::Forms::Button^  btnRetry;
	protected: 

	private:
		/// <summary>
		/// Erforderliche Designervariable.
		/// </summary>
		System::ComponentModel::Container ^components;

#pragma region Windows Form Designer generated code
		/// <summary>
		/// Erforderliche Methode für die Designerunterstützung.
		/// Der Inhalt der Methode darf nicht mit dem Code-Editor geändert werden.
		/// </summary>
		void InitializeComponent(void)
		{
			System::ComponentModel::ComponentResourceManager^  resources = (gcnew System::ComponentModel::ComponentResourceManager(formError::typeid));
			this->lblError = (gcnew System::Windows::Forms::Label());
			this->btnRetry = (gcnew System::Windows::Forms::Button());
			this->SuspendLayout();
			// 
			// lblError
			// 
			this->lblError->ForeColor = System::Drawing::Color::FromArgb(static_cast<System::Int32>(static_cast<System::Byte>(187)), static_cast<System::Int32>(static_cast<System::Byte>(187)), 
				static_cast<System::Int32>(static_cast<System::Byte>(187)));
			this->lblError->Location = System::Drawing::Point(50, 43);
			this->lblError->Name = L"lblError";
			this->lblError->Size = System::Drawing::Size(300, 75);
			this->lblError->TabIndex = 5;
			this->lblError->Text = L"Error";
			this->lblError->TextAlign = System::Drawing::ContentAlignment::TopCenter;
			this->lblError->Click += gcnew System::EventHandler(this, &formError::lblError_Click);
			// 
			// btnRetry
			// 
			this->btnRetry->ForeColor = System::Drawing::Color::Black;
			this->btnRetry->Location = System::Drawing::Point(156, 138);
			this->btnRetry->Name = L"btnRetry";
			this->btnRetry->Size = System::Drawing::Size(86, 23);
			this->btnRetry->TabIndex = 8;
			this->btnRetry->Text = L"&retry";
			this->btnRetry->UseVisualStyleBackColor = true;
			this->btnRetry->Click += gcnew System::EventHandler(this, &formError::btnRetry_Click);
			// 
			// formError
			// 
			this->AutoScaleDimensions = System::Drawing::SizeF(8, 16);
			this->AutoScaleMode = System::Windows::Forms::AutoScaleMode::Font;
			this->BackColor = System::Drawing::Color::White;
			this->ClientSize = System::Drawing::Size(404, 562);
			this->Controls->Add(this->btnRetry);
			this->Controls->Add(this->lblError);
			this->Font = (gcnew System::Drawing::Font(L"Microsoft Sans Serif", 13, System::Drawing::FontStyle::Regular, System::Drawing::GraphicsUnit::Pixel));
			this->Icon = (cli::safe_cast<System::Drawing::Icon^  >(resources->GetObject(L"$this.Icon")));
			this->Margin = System::Windows::Forms::Padding(4);
			this->Name = L"formError";
			this->Text = L"qaul.net – قول";
			this->Load += gcnew System::EventHandler(this, &formError::formError_Load);
			this->ResumeLayout(false);

		}
#pragma endregion
	private: System::Void formError_Load(System::Object^  sender, System::EventArgs^  e) {
			 }
	private: System::Void btnRetry_Click(System::Object^  sender, System::EventArgs^  e) {
			 }
	private: System::Void lblError_Click(System::Object^  sender, System::EventArgs^  e) {
			 }
	};
}

describe('It Updates State Properly ', () => {
	it('updates parent', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=update-parent]').click();
		cy.get('[data-cy=main-text]').should('have.text', 'Main -100');
	});

	it('updates main', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=increment-main]').click();
		cy.get('[data-cy=main-text]').should('have.text', 'Main 100');
	});

	it('updates main-child', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=increment-main-child]').click();
		cy.get('[data-cy=main-child-text]').should('have.text', 'Main Child 1');
	});

	it('updates main-child-sibling', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=increment-main-child-sibling]').click();
		cy.get('[data-cy=main-child-sibling-text]').should('have.text', 'Main Child Sibling 1');
	});

	it('updates grandchild', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=decrement-grandchild]').click();
		cy.get('[data-cy=grandchild-text]').should('have.text', 'GrandChild -1');
	});

	it('main sends updates to grandchild', () => {
		cy.visit('http://localhost:8000/');
		const typedText = 'test@email.com';
		cy.get('#input-el').type(typedText);
		cy.get('[data-cy=send-input-val]').click();
		cy.get('[data-cy=grandchild-props-text]').should('have.text', typedText);
	});

	it('components hold state properly', () => {
		cy.visit('http://localhost:8000/');
		const typedText = 'test@email.com';
		cy.get('#input-el').type(typedText);
		cy.get('[data-cy=send-input-val]').click();
		cy.get('[data-cy=grandchild-props-text]').should('have.text', typedText);
		cy.get('[data-cy=update-parent]').click();
		cy.get('[data-cy=increment-main]').click();
		cy.get('[data-cy=increment-main]').click();
		cy.get('[data-cy=increment-main-child]').click();
		cy.get('[data-cy=increment-main-child-sibling]').click();
		cy.get('[data-cy=decrement-grandchild]').click();
		cy.get('[data-cy=increment-main]').click();
		cy.get('[data-cy=update-parent]').click();
		cy.get('[data-cy=increment-main-child]').click();

		cy.get('[data-cy=main-child-text]').should('have.text', 'Main Child 2');
		cy.get('[data-cy=main-child-sibling-text]').should('have.text', 'Main Child Sibling 1');
		cy.get('[data-cy=grandchild-text]').should('have.text', 'GrandChild -1');
		cy.get('[data-cy=main-text]').should('have.text', 'Main 100');
	});
});
